use serde_json::{json, Value};
use std::{process::Output, time::Duration};
use tempfile::TempDir;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
    process::Command,
    task::JoinHandle,
    time::timeout,
};

const MARKER: &str = "SenseNova-Volt-Integration-Success";

fn probe_command(workspace: &std::path::Path) -> Command {
    let mut command = Command::new(env!("CARGO_BIN_EXE_sensenova-live-test"));
    command
        .current_dir(workspace)
        .env_remove("SENSENOVA_API_KEY")
        .env_remove("OPENAI_API_KEY")
        .env_remove("OPENAI_BASE_URL")
        .env_remove("HTTP_PROXY")
        .env_remove("HTTPS_PROXY")
        .env_remove("ALL_PROXY")
        .env_remove("http_proxy")
        .env_remove("https_proxy")
        .env_remove("all_proxy")
        .kill_on_drop(true);
    command
}

async fn probe_output(command: &mut Command) -> Output {
    timeout(Duration::from_secs(15), command.output())
        .await
        .expect("live probe did not terminate")
        .unwrap()
}

fn assert_rejected(output: &Output) {
    assert!(!output.status.success(), "{output:?}");
    assert!(!String::from_utf8_lossy(&output.stdout).contains("PASSED"));
}

struct ProbeFixture {
    workspace: TempDir,
    base_url: String,
    server: JoinHandle<Vec<Value>>,
}

impl Drop for ProbeFixture {
    fn drop(&mut self) {
        self.server.abort();
    }
}

async fn read_request(socket: &mut TcpStream) -> Value {
    let mut request = Vec::new();
    loop {
        let mut buffer = [0; 4096];
        let count = socket.read(&mut buffer).await.unwrap();
        assert!(count > 0, "probe closed before completing its request");
        request.extend_from_slice(&buffer[..count]);
        assert!(request.len() <= 1024 * 1024);
        if let Some(end) = request.windows(4).position(|part| part == b"\r\n\r\n") {
            let headers = String::from_utf8_lossy(&request[..end]).to_lowercase();
            let length: usize = headers
                .lines()
                .find_map(|line| line.strip_prefix("content-length:"))
                .unwrap()
                .trim()
                .parse()
                .unwrap();
            if request.len() >= end + 4 + length {
                return serde_json::from_slice(&request[end + 4..end + 4 + length]).unwrap();
            }
        }
    }
}

impl ProbeFixture {
    async fn start(bodies: Vec<String>) -> Self {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let base_url = format!("http://{}/v1", listener.local_addr().unwrap());
        let server = tokio::spawn(async move {
            let mut requests = Vec::new();
            for body in bodies {
                let (mut socket, _) = listener.accept().await.unwrap();
                requests.push(read_request(&mut socket).await);
                let response = format!(
                    "HTTP/1.1 200 OK\r\nContent-Type: text/event-stream\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",
                    body.len()
                );
                socket.write_all(response.as_bytes()).await.unwrap();
            }
            requests
        });
        Self {
            workspace: tempfile::tempdir().unwrap(),
            base_url,
            server,
        }
    }

    fn command(&self) -> Command {
        let mut command = probe_command(self.workspace.path());
        command
            .env("OPENAI_API_KEY", "synthetic-live-probe-key")
            .env("OPENAI_BASE_URL", &self.base_url);
        command
    }

    async fn run(&self) -> Output {
        probe_output(&mut self.command()).await
    }
}

fn response(delta: Value) -> String {
    format!(
        "data: {}\n\ndata: [DONE]\n\n",
        json!({"choices":[{"delta":delta}]})
    )
}

#[cfg(unix)]
fn bash_response(command: &str) -> String {
    response(json!({"tool_calls":[{
        "index":0, "id":"probe-call", "type":"function",
        "function":{"name":"bash","arguments":json!({"command":command}).to_string()}
    }]}))
}

#[tokio::test]
async fn missing_or_blank_credentials_are_not_success() {
    // Both unset and whitespace-only credentials must fail before any provider request.
    for key in [None, Some(""), Some(" \t")] {
        let workspace = tempfile::tempdir().unwrap();
        let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
        listener.set_nonblocking(true).unwrap();
        let mut command = probe_command(workspace.path());
        command.env(
            "OPENAI_BASE_URL",
            format!("http://{}/v1", listener.local_addr().unwrap()),
        );
        if let Some(key) = key {
            command.env("SENSENOVA_API_KEY", key);
        }
        let output = probe_output(&mut command).await;
        assert_rejected(&output);
        assert!(String::from_utf8_lossy(&output.stderr).contains("API_KEY"));
        assert!(
            matches!(listener.accept(), Err(error) if error.kind() == std::io::ErrorKind::WouldBlock),
            "probe contacted the provider without a credential"
        );
    }
}

#[tokio::test]
async fn model_text_cannot_substitute_for_a_tool_result() {
    let fixture = ProbeFixture::start(vec![response(json!({"content":MARKER}))]).await;
    assert_rejected(&fixture.run().await);
}

#[tokio::test]
async fn malformed_provider_stream_cannot_pass() {
    let fixture = ProbeFixture::start(vec!["data: {invalid}\n\ndata: [DONE]\n\n".into()]).await;
    assert_rejected(&fixture.run().await);
}

#[cfg(unix)]
#[tokio::test]
async fn provider_failure_after_verified_tool_output_cannot_pass() {
    let fixture = ProbeFixture::start(vec![
        bash_response(&format!("printf '{MARKER}\\n'")),
        "data: {invalid}\n\ndata: [DONE]\n\n".into(),
    ])
    .await;
    let output = fixture.run().await;
    assert!(String::from_utf8_lossy(&output.stdout).contains("[TOOL CALL FINISHED] bash"));
    assert_rejected(&output);
}

#[cfg(unix)]
#[tokio::test]
async fn requested_shell_result_must_succeed_and_match_exactly() {
    // Wrong output, marker plus extra output, and a failing process are distinct failures.
    for command in [
        "printf unexpected".to_string(),
        format!("printf '{MARKER} extra'"),
        format!("printf '{MARKER}'; exit 7"),
    ] {
        let fixture = ProbeFixture::start(vec![
            bash_response(&command),
            response(json!({"content":"Finished"})),
        ])
        .await;
        assert_rejected(&fixture.run().await);
    }
}

#[cfg(unix)]
#[tokio::test]
async fn successful_probe_returns_real_tool_output_to_the_provider() {
    let mut fixture = ProbeFixture::start(vec![
        bash_response(&format!("printf '{MARKER}\\n'")),
        response(json!({"content":"Verified"})),
    ])
    .await;
    let output = probe_output(fixture.command().env("SENSENOVA_API_KEY", " \t")).await;
    assert!(output.status.success(), "{output:?}");
    assert!(String::from_utf8_lossy(&output.stdout).contains("PASSED"));
    let requests = timeout(Duration::from_secs(5), &mut fixture.server)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(requests.len(), 2);
    assert!(requests[1]["messages"]
        .as_array()
        .unwrap()
        .iter()
        .any(|message| message["role"] == "tool"
            && message["tool_call_id"] == "probe-call"
            && message["content"] == format!("{MARKER}\n")));
}
