use pi_agent_core::{agent::ToolCheckpoint, AgentEvent, Message, ModelConfig, PiAgent};
use serde_json::json;
use std::time::Duration;
use tempfile::TempDir;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpListener,
    sync::mpsc,
    task::JoinHandle,
};

struct CheckpointRun {
    workspace: TempDir,
    checkpoints: mpsc::Receiver<ToolCheckpoint>,
    events: mpsc::Receiver<AgentEvent>,
    worker: JoinHandle<(anyhow::Result<()>, Vec<Message>)>,
    server: JoinHandle<()>,
}

impl Drop for CheckpointRun {
    fn drop(&mut self) {
        self.worker.abort();
        self.server.abort();
    }
}

impl CheckpointRun {
    async fn start(tool_names: &[&str]) -> Self {
        let workspace = tempfile::tempdir().unwrap();
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let base = format!("http://{}/v1", listener.local_addr().unwrap());
        let tool_calls: Vec<_> = tool_names
            .iter()
            .enumerate()
            .map(|(index, name)| {
                json!({
                    "index": index, "id": format!("call-{index}"), "type": "function",
                    "function": {
                        "name": name,
                        "arguments": json!({
                            "path": format!("{index}.txt"), "content": "checkpointed"
                        }).to_string()
                    }
                })
            })
            .collect();
        let server = tokio::spawn(async move {
            for delta in [
                json!({"tool_calls": tool_calls}),
                json!({"content": "Finished"}),
            ] {
                let (mut socket, _) = listener.accept().await.unwrap();
                let mut request = Vec::new();
                loop {
                    let mut buffer = [0; 4096];
                    let count = socket.read(&mut buffer).await.unwrap();
                    if count == 0 {
                        return;
                    }
                    request.extend_from_slice(&buffer[..count]);
                    if let Some(end) = request.windows(4).position(|part| part == b"\r\n\r\n") {
                        let length: usize = String::from_utf8_lossy(&request[..end])
                            .lines()
                            .find_map(|line| {
                                line.to_lowercase()
                                    .strip_prefix("content-length:")
                                    .map(|length| length.trim().parse().unwrap())
                            })
                            .unwrap();
                        if request.len() >= end + 4 + length {
                            break;
                        }
                    }
                }
                let body = format!(
                    "data: {}\n\ndata: [DONE]\n\n",
                    json!({"choices":[{"delta":delta}]})
                );
                let response = format!(
                    "HTTP/1.1 200 OK\r\nContent-Type: text/event-stream\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",
                    body.len()
                );
                if socket.write_all(response.as_bytes()).await.is_err() {
                    return;
                }
            }
        });
        let mut agent = PiAgent::new(
            workspace.path(),
            ModelConfig {
                api_base: Some(base),
                api_key: None,
                ..Default::default()
            },
        );
        let (checkpoint_tx, checkpoints) = mpsc::channel(1);
        agent.tool_checkpoints = Some(checkpoint_tx);
        let (event_tx, events) = mpsc::channel(100);
        let worker = tokio::spawn(async move {
            let mut messages = vec![Message::user("Run the fixture tools")];
            let outcome = agent.run_turn(&mut messages, event_tx).await;
            (outcome, messages)
        });
        Self {
            workspace,
            checkpoints,
            events,
            worker,
            server,
        }
    }

    async fn checkpoint(&mut self) -> ToolCheckpoint {
        tokio::time::timeout(Duration::from_secs(5), self.checkpoints.recv())
            .await
            .unwrap()
            .unwrap()
    }

    async fn finish(&mut self) -> (anyhow::Result<()>, Vec<Message>) {
        tokio::time::timeout(Duration::from_secs(5), &mut self.worker)
            .await
            .unwrap()
            .unwrap()
    }

    fn assert_not_started(&self) {
        assert!(!self.workspace.path().join("0.txt").exists());
        assert!(!self.workspace.path().join("1.txt").exists());
    }
}

#[tokio::test]
async fn each_real_write_waits_for_its_own_history_acknowledgement() {
    let mut run = CheckpointRun::start(&["write_file", "write_file"]).await;
    let first = run.checkpoint().await;
    assert_eq!(first.call.id, "call-0");
    run.assert_not_started();
    first.acknowledgement.send(Ok(())).unwrap();

    let second = run.checkpoint().await;
    assert_eq!(second.call.id, "call-1");
    assert_eq!(
        std::fs::read_to_string(run.workspace.path().join("0.txt")).unwrap(),
        "checkpointed"
    );
    assert!(!run.workspace.path().join("1.txt").exists());
    second.acknowledgement.send(Ok(())).unwrap();

    let (outcome, messages) = run.finish().await;
    outcome.unwrap();
    assert_eq!(
        std::fs::read_to_string(run.workspace.path().join("1.txt")).unwrap(),
        "checkpointed"
    );
    assert_eq!(messages.last().unwrap().content, "Finished");
}

#[tokio::test]
async fn rejected_history_prevents_execution_and_records_known_not_started() {
    for name in ["write_file", "bash", "delegate_agents"] {
        let mut run = CheckpointRun::start(&[name, "write_file"]).await;
        let checkpoint = run.checkpoint().await;
        assert_eq!(checkpoint.call.name, name);
        checkpoint
            .acknowledgement
            .send(Err(anyhow::anyhow!("State changed in another instance")))
            .unwrap();

        let (outcome, messages) = run.finish().await;
        let error = format!("{:#}", outcome.unwrap_err());
        assert!(error.contains("Tool was not started"));
        assert!(error.contains("State changed in another instance"));
        run.assert_not_started();
        let result = messages.last().unwrap().tool_result.as_ref().unwrap();
        assert!(result.is_error);
        assert_eq!(result.tool_call_id, "call-0");
        assert!(result.output.contains("Tool was not started"));
        assert!(run.checkpoints.recv().await.is_none());
        assert!(
            std::iter::from_fn(|| run.events.try_recv().ok()).any(|event| {
                matches!(event, AgentEvent::ToolCallFinished { is_error: true, output, .. }
                if output.contains("Tool was not started"))
            })
        );
    }
}

#[tokio::test]
async fn lost_checkpoint_acknowledgement_prevents_execution() {
    let mut run = CheckpointRun::start(&["write_file"]).await;
    drop(run.checkpoint().await);
    let (outcome, _) = run.finish().await;
    assert!(format!("{:#}", outcome.unwrap_err()).contains("acknowledgement closed"));
    run.assert_not_started();
}

#[tokio::test]
async fn closed_checkpoint_receiver_prevents_execution() {
    let mut run = CheckpointRun::start(&["write_file"]).await;
    run.checkpoints.close();
    let (outcome, _) = run.finish().await;
    assert!(format!("{:#}", outcome.unwrap_err()).contains("receiver closed"));
    run.assert_not_started();
}

#[tokio::test]
async fn closed_event_receiver_prevents_tool_execution() {
    let mut run = CheckpointRun::start(&["write_file"]).await;
    run.events.close();
    let (outcome, _) = run.finish().await;
    assert!(format!("{:#}", outcome.unwrap_err()).contains("event receiver closed"));
    run.assert_not_started();
}

#[tokio::test]
async fn cancelled_run_cannot_execute_after_a_late_acknowledgement() {
    let mut run = CheckpointRun::start(&["write_file"]).await;
    let checkpoint = run.checkpoint().await;
    run.worker.abort();
    assert!((&mut run.worker).await.unwrap_err().is_cancelled());
    assert!(checkpoint.acknowledgement.send(Ok(())).is_err());
    run.assert_not_started();
}

#[tokio::test]
async fn stalled_checkpoint_times_out_without_executing() {
    let mut run = CheckpointRun::start(&["write_file"]).await;
    let checkpoint = run.checkpoint().await;
    let (outcome, _) = tokio::time::timeout(Duration::from_secs(35), &mut run.worker)
        .await
        .unwrap()
        .unwrap();
    assert!(format!("{:#}", outcome.unwrap_err()).contains("checkpoint timed out"));
    assert!(checkpoint.acknowledgement.send(Ok(())).is_err());
    run.assert_not_started();
}
