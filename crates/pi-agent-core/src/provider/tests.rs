use super::{request_headers, request_url, LlmClient, RequestTimeouts};
use crate::{AgentEvent, Message, ModelConfig};
use reqwest::header::AUTHORIZATION;
use std::time::Duration;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
    sync::{mpsc, oneshot},
    task::JoinHandle,
};

#[derive(Clone, Copy, PartialEq)]
enum Stall {
    Headers,
    Body,
    Heartbeats,
    ErrorBodyHeartbeats,
}

struct StalledServer {
    base: String,
    closed: oneshot::Receiver<()>,
    task: JoinHandle<()>,
}

impl Drop for StalledServer {
    fn drop(&mut self) {
        self.task.abort();
    }
}

impl StalledServer {
    async fn start(stall: Stall) -> Self {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let base = format!("http://{}/v1", listener.local_addr().unwrap());
        let (closed_tx, closed) = oneshot::channel();
        let task = tokio::spawn(async move {
            let (mut socket, _) = listener.accept().await.unwrap();
            read_request(&mut socket).await;
            if stall == Stall::ErrorBodyHeartbeats {
                socket
                    .write_all(b"HTTP/1.1 429 Too Many Requests\r\nConnection: close\r\n\r\n")
                    .await
                    .unwrap();
            } else if stall != Stall::Headers {
                socket.write_all(b"HTTP/1.1 200 OK\r\nContent-Type: text/event-stream\r\nConnection: close\r\n\r\ndata: {\"choices\":[{\"delta\":{\"content\":\"partial\"}}]}\n\n").await.unwrap();
            }
            await_disconnect(&mut socket, stall).await;
            let _ = closed_tx.send(());
        });
        Self { base, closed, task }
    }
}

async fn read_request(socket: &mut TcpStream) {
    let mut request = Vec::new();
    loop {
        let mut chunk = [0; 4096];
        let count = socket.read(&mut chunk).await.unwrap();
        assert!(count > 0);
        request.extend_from_slice(&chunk[..count]);
        if let Some(end) = request.windows(4).position(|bytes| bytes == b"\r\n\r\n") {
            let headers = String::from_utf8_lossy(&request[..end]).to_lowercase();
            let size: usize = headers
                .lines()
                .find_map(|line| line.strip_prefix("content-length:"))
                .unwrap()
                .trim()
                .parse()
                .unwrap();
            if request.len() >= end + 4 + size {
                return;
            }
        }
    }
}

async fn await_disconnect(socket: &mut TcpStream, stall: Stall) {
    let mut interval = tokio::time::interval(Duration::from_millis(10));
    let mut byte = [0; 1];
    loop {
        tokio::select! {
            read = socket.read(&mut byte) => {
                if read.is_err() || read.unwrap() == 0 {
                    return;
                }
            }
            _ = interval.tick(), if matches!(stall, Stall::Heartbeats | Stall::ErrorBodyHeartbeats) => {
                if socket.write_all(b": heartbeat\n\n").await.is_err() {
                    return;
                }
            }
        }
    }
}

async fn assert_stall_is_cancelled(stall: Stall, timeouts: RequestTimeouts, expected: &str) {
    let mut server = StalledServer::start(stall).await;
    let config = ModelConfig {
        api_base: Some(server.base.clone()),
        api_key: None,
        ..Default::default()
    };
    let (tx, mut rx) = mpsc::channel(16);
    let response = tokio::time::timeout(
        Duration::from_secs(3),
        LlmClient::with_timeouts(timeouts).stream_chat(&config, &[Message::user("test")], None, tx),
    )
    .await
    .expect("client did not enforce its deadline");
    let error = response.unwrap_err();
    assert!(error.to_string().contains(expected), "{error:#}");
    let mut partial = false;
    let mut http_failure = false;
    while let Some(event) = rx.recv().await {
        assert!(!matches!(event, AgentEvent::Completed { .. }));
        match event {
            AgentEvent::TextDelta(text) => partial |= text == "partial",
            AgentEvent::Error(error) => http_failure |= error.contains("429"),
            _ => {}
        }
    }
    assert_eq!(partial, matches!(stall, Stall::Body | Stall::Heartbeats));
    assert_eq!(http_failure, stall == Stall::ErrorBodyHeartbeats);
    tokio::time::timeout(Duration::from_secs(1), &mut server.closed)
        .await
        .expect("timed-out request left the connection open")
        .unwrap();
}

#[tokio::test]
async fn response_headers_have_an_independent_deadline() {
    assert_stall_is_cancelled(
        Stall::Headers,
        RequestTimeouts {
            total: Duration::from_secs(2),
            headers: Duration::from_millis(200),
            idle: Duration::from_secs(1),
        },
        "response headers timed out",
    )
    .await;
}

#[tokio::test]
async fn stalled_body_retains_partial_output_and_closes_the_connection() {
    assert_stall_is_cancelled(
        Stall::Body,
        RequestTimeouts {
            total: Duration::from_secs(2),
            headers: Duration::from_secs(1),
            idle: Duration::from_millis(200),
        },
        "stream stalled",
    )
    .await;
}

#[tokio::test]
async fn continuous_heartbeats_cannot_extend_the_total_deadline() {
    assert_stall_is_cancelled(
        Stall::Heartbeats,
        RequestTimeouts {
            total: Duration::from_millis(500),
            headers: Duration::from_millis(200),
            idle: Duration::from_millis(200),
        },
        "deadline",
    )
    .await;
}

#[tokio::test]
async fn a_slow_error_body_cannot_replace_the_known_http_status() {
    assert_stall_is_cancelled(
        Stall::ErrorBodyHeartbeats,
        RequestTimeouts {
            total: Duration::from_millis(500),
            headers: Duration::from_millis(200),
            idle: Duration::from_millis(200),
        },
        "429",
    )
    .await;
}

#[test]
fn credential_headers_fail_closed_and_remain_sensitive() {
    const TEST_CREDENTIAL: &str = "test-only-not-a-real-credential";
    let headers = request_headers(Some(TEST_CREDENTIAL)).unwrap();
    assert!(headers[AUTHORIZATION].is_sensitive());
    assert!(!format!("{headers:?}").contains(TEST_CREDENTIAL));
    for invalid in ["", "   ", "test\ninvalid"] {
        assert!(request_headers(Some(invalid)).is_err());
    }
    assert!(!request_headers(None).unwrap().contains_key(AUTHORIZATION));
}

#[test]
fn endpoint_paths_are_normalized_and_credentials_are_rejected() {
    for (base, expected) in [
        (
            "http://localhost:1234/v1",
            "http://localhost:1234/v1/chat/completions",
        ),
        (
            "https://example.com/v1/chat/completions/",
            "https://example.com/v1/chat/completions",
        ),
        (
            "https://example.com/a%20b/",
            "https://example.com/a%20b/chat/completions",
        ),
    ] {
        assert_eq!(request_url(Some(base)).unwrap().as_str(), expected);
    }
    for base in [
        "file:///tmp/api",
        "https://user@example.com",
        "https://example.com?key=test",
        "https://example.com#fragment",
    ] {
        assert!(request_url(Some(base)).is_err());
    }
}

async fn http_response(status: &str, body: String) -> (String, JoinHandle<()>) {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let base = format!("http://{}/v1", listener.local_addr().unwrap());
    let headers = format!(
        "HTTP/1.1 {status}\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
        body.len()
    );
    let task = tokio::spawn(async move {
        let (mut socket, _) = listener.accept().await.unwrap();
        read_request(&mut socket).await;
        socket.write_all(headers.as_bytes()).await.unwrap();
        let _ = socket.write_all(body.as_bytes()).await;
    });
    (base, task)
}

#[tokio::test]
async fn provider_errors_cannot_echo_the_api_credential() {
    const TEST_CREDENTIAL: &str = "test-only-not-a-real-credential";
    for (status, body) in [
        ("401 Unauthorized", format!("Rejected credential {TEST_CREDENTIAL}")),
        ("200 OK", format!("data: {{\"choices\":[{{\"delta\":{{\"tool_calls\":[{{\"index\":\"{TEST_CREDENTIAL}\"}}]}}}}]}}\n\ndata: [DONE]\n\n")),
    ] {
        let (base, task) = http_response(status, body).await;
        let config = ModelConfig {
            api_base: Some(base),
            api_key: Some(TEST_CREDENTIAL.into()),
            ..Default::default()
        };
        let (tx, mut rx) = mpsc::channel(16);
        let error = LlmClient::new()
            .stream_chat(&config, &[Message::user("test")], None, tx)
            .await
            .unwrap_err();
        assert!(!format!("{error:#}").contains(TEST_CREDENTIAL));
        while let Some(event) = rx.recv().await {
            assert!(!format!("{event:?}").contains(TEST_CREDENTIAL));
            assert!(!matches!(event, AgentEvent::Completed { .. }));
        }
        task.await.unwrap();
    }
}

#[tokio::test]
async fn error_response_size_is_bounded_without_losing_http_failure() {
    let (base, task) = http_response("502 Bad Gateway", "x".repeat(8 * 1024 + 1)).await;
    let config = ModelConfig {
        api_base: Some(base),
        api_key: None,
        ..Default::default()
    };
    let (tx, _) = mpsc::channel(16);
    let error = LlmClient::new()
        .stream_chat(&config, &[Message::user("test")], None, tx)
        .await
        .unwrap_err();
    assert!(error.to_string().contains("502"));
    assert!(error.to_string().contains("8 KiB"));
    task.await.unwrap();
}
