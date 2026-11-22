use pi_agent_core::{provider::LlmClient, AgentEvent, Message, ModelConfig};
use std::time::Duration;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpListener,
    sync::mpsc,
};

async fn serve(body: Vec<u8>, split: bool) -> (String, tokio::task::JoinHandle<()>) {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address = listener.local_addr().unwrap();
    let worker = tokio::spawn(async move {
        let (mut socket, _) = listener.accept().await.unwrap();
        let mut request = Vec::new();
        loop {
            let mut buf = [0; 4096];
            let count = socket.read(&mut buf).await.unwrap();
            assert!(count > 0);
            request.extend_from_slice(&buf[..count]);
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
                    break;
                }
            }
        }
        socket.write_all(b"HTTP/1.1 200 OK\r\nContent-Type: text/event-stream\r\nTransfer-Encoding: chunked\r\n\r\n").await.unwrap();
        for chunk in body.chunks(if split { 1 } else { body.len() }) {
            let frame = format!("{:x}\r\n", chunk.len());
            if socket.write_all(frame.as_bytes()).await.is_err() {
                return;
            }
            if socket.write_all(chunk).await.is_err() {
                return;
            }
            if socket.write_all(b"\r\n").await.is_err() {
                return;
            }
            if split {
                tokio::time::sleep(Duration::from_millis(1)).await;
            }
        }
        let _ = socket.write_all(b"0\r\n\r\n").await;
    });
    (format!("http://{address}/v1"), worker)
}

#[tokio::test]
async fn chinese_deltas_survive_split_utf8_packets() {
    let body = "data: {\"choices\":[{\"delta\":{\"content\":\"中文恢复🙂\"}}]}\n\ndata: [DONE]\n\n";
    let (base, server) = serve(body.as_bytes().to_vec(), true).await;
    let config = ModelConfig {
        api_base: Some(base),
        ..Default::default()
    };
    let (tx, mut rx) = mpsc::channel(16);
    let message = LlmClient::new()
        .stream_chat(&config, &[Message::user("test")], None, tx)
        .await
        .unwrap();
    assert_eq!(message.content, "中文恢复🙂");
    assert!(matches!(rx.recv().await,Some(AgentEvent::TextDelta(text)) if text=="中文恢复🙂"));
    server.await.unwrap();
}

#[tokio::test]
async fn disconnected_or_malformed_streams_do_not_report_success() {
    for body in [
        "data: {\"choices\":[{\"delta\":{\"content\":\"partial\"}}]}\n\n",
        "data: {invalid json}\n\ndata: [DONE]\n\n",
        "data: {\"error\":{\"message\":\"failed\"}}\n\n",
        "data: {\"choices\":[{\"finish_reason\":false}]}\n\n",
        "data: {\"choices\":[{\"finish_reason\":\"length\"}]}\n\n",
    ] {
        let (base, server) = serve(body.as_bytes().to_vec(), false).await;
        let config = ModelConfig {
            api_base: Some(base),
            ..Default::default()
        };
        let (tx, mut rx) = mpsc::channel(16);
        assert!(LlmClient::new()
            .stream_chat(&config, &[Message::user("test")], None, tx)
            .await
            .is_err());
        while let Some(event) = rx.recv().await {
            assert!(!matches!(event, AgentEvent::Completed { .. }));
        }
        server.await.unwrap();
    }
}

#[tokio::test]
async fn malformed_deltas_and_tool_identities_never_complete_successfully() {
    for delta in [
        r#"{"content":{"text":"silently dropped"}}"#,
        r#"{"reasoning_content":42}"#,
        r#"{"tool_calls":{}}"#,
        r#"{"tool_calls":[{"index":"0","id":"call","function":{"name":"calc","arguments":"{}"}}]}"#,
        r#"{"tool_calls":[{"index":-1,"id":"call","function":{"name":"calc","arguments":"{}"}}]}"#,
        r#"{"tool_calls":[{"index":1.5,"id":"call","function":{"name":"calc","arguments":"{}"}}]}"#,
        r#"{"tool_calls":[{"id":"call","function":{"name":"calc","arguments":"{}"}}]}"#,
        r#"{"tool_calls":[{"index":0,"id":"call","type":"unknown","function":{"name":"calc","arguments":"{}"}}]}"#,
        r#"{"tool_calls":[{"index":0,"id":"call","function":{"name":"calc","arguments":"null"}}]}"#,
        r#"{"tool_calls":[{"index":0,"id":"call","function":{"name":"calc","arguments":"[]"}}]}"#,
        r#"{"tool_calls":[{"index":0,"id":"same","function":{"name":"calc","arguments":"{}"}},{"index":1,"id":"same","function":{"name":"calc","arguments":"{}"}}]}"#,
    ] {
        let body = format!("data: {{\"choices\":[{{\"delta\":{delta}}}]}}\n\ndata: [DONE]\n\n");
        let (base, server) = serve(body.into_bytes(), false).await;
        let config = ModelConfig {
            api_base: Some(base),
            api_key: None,
            ..Default::default()
        };
        let (tx, mut rx) = mpsc::channel(16);
        let response = LlmClient::new()
            .stream_chat(&config, &[Message::user("test")], None, tx)
            .await;
        assert!(response.is_err(), "Accepted malformed delta: {delta}");
        while let Some(event) = rx.recv().await {
            assert!(!matches!(event, AgentEvent::Completed { .. }));
        }
        server.await.unwrap();
    }
}

#[tokio::test]
async fn multiline_events_and_all_line_endings_survive_byte_splitting() {
    for newline in ["\n", "\r\n", "\r"] {
        let body = [
            "\u{feff}: first comment",
            "event: message",
            "data: {",
            "data: \"choices\":[{\"delta\":{\"content\":\"多行🙂\"}}]}",
            "",
            "data: [DONE]",
            "",
            "",
        ]
        .join(newline);
        let (base, server) = serve(body.into_bytes(), true).await;
        let config = ModelConfig {
            api_base: Some(base),
            api_key: None,
            ..Default::default()
        };
        let (tx, mut rx) = mpsc::channel(16);
        let message = LlmClient::new()
            .stream_chat(&config, &[Message::user("test")], None, tx)
            .await
            .unwrap();
        assert_eq!(message.content, "多行🙂");
        assert!(matches!(rx.recv().await, Some(AgentEvent::TextDelta(text)) if text == "多行🙂"));
        assert!(matches!(
            rx.recv().await,
            Some(AgentEvent::Completed { .. })
        ));
        server.await.unwrap();
    }
}

#[tokio::test]
async fn interleaved_tool_fragments_and_usage_preserve_their_identity() {
    let frames = [
        serde_json::json!({"choices":[{"delta":{"reasoning_content":"checking","tool_calls":[{"index":1,"id":"second","type":"function","function":{"name":"calc","arguments":"{\"action\":"}}]}}]}),
        serde_json::json!({"choices":[{"delta":{"tool_calls":[{"index":0,"id":"first","function":{"name":"read_file","arguments":"{\"path\":"}}]}}]}),
        serde_json::json!({"choices":[{"delta":{"tool_calls":[{"index":1,"function":{"arguments":"\"eval\",\"expression\":\"1+1\"}"}}]}}]}),
        serde_json::json!({"choices":[{"delta":{"tool_calls":[{"index":0,"function":{"arguments":"\"src/lib.rs\"}"}}]},"finish_reason":"tool_calls"}]}),
        serde_json::json!({"choices":[],"usage":{"total_tokens":23}}),
    ];
    let mut body = frames
        .iter()
        .map(|frame| format!("data: {frame}\n\n"))
        .collect::<String>();
    body.push_str("data: [DONE]\n\n");
    let (base, server) = serve(body.into_bytes(), false).await;
    let config = ModelConfig {
        api_base: Some(base),
        api_key: None,
        ..Default::default()
    };
    let (tx, mut rx) = mpsc::channel(16);
    let message = LlmClient::new()
        .stream_chat(&config, &[Message::user("test")], None, tx)
        .await
        .unwrap();
    assert_eq!(message.reasoning.as_deref(), Some("checking"));
    let calls = message.tool_calls.unwrap();
    assert_eq!(calls.len(), 2);
    assert_eq!(
        (calls[0].id.as_str(), calls[0].name.as_str()),
        ("first", "read_file")
    );
    assert_eq!(calls[0].arguments, serde_json::json!({"path":"src/lib.rs"}));
    assert_eq!(
        (calls[1].id.as_str(), calls[1].name.as_str()),
        ("second", "calc")
    );
    assert_eq!(
        calls[1].arguments,
        serde_json::json!({"action":"eval","expression":"1+1"})
    );
    assert!(matches!(rx.recv().await, Some(AgentEvent::Thought(text)) if text == "checking"));
    assert!(matches!(
        rx.recv().await,
        Some(AgentEvent::Completed {
            total_tokens: Some(23)
        })
    ));
    server.await.unwrap();
}

#[tokio::test]
async fn partial_text_survives_a_later_malformed_event_in_the_same_packet() {
    let body =
        b"data: {\"choices\":[{\"delta\":{\"content\":\"retained\"}}]}\n\ndata: {invalid}\n\n";
    let (base, server) = serve(body.to_vec(), false).await;
    let config = ModelConfig {
        api_base: Some(base),
        api_key: None,
        ..Default::default()
    };
    let (tx, mut rx) = mpsc::channel(16);
    assert!(LlmClient::new()
        .stream_chat(&config, &[Message::user("test")], None, tx)
        .await
        .is_err());
    assert!(matches!(rx.recv().await, Some(AgentEvent::TextDelta(text)) if text == "retained"));
    assert!(rx.recv().await.is_none());
    server.await.unwrap();
}

#[tokio::test]
async fn trailing_truncated_events_cannot_reuse_an_earlier_finish_reason() {
    let body = b"data: {\"choices\":[{\"delta\":{\"content\":\"partial\"},\"finish_reason\":\"stop\"}]}\n\ndata: {";
    let (base, server) = serve(body.to_vec(), false).await;
    let config = ModelConfig {
        api_base: Some(base),
        api_key: None,
        ..Default::default()
    };
    let (tx, mut rx) = mpsc::channel(16);
    assert!(LlmClient::new()
        .stream_chat(&config, &[Message::user("test")], None, tx)
        .await
        .is_err());
    while let Some(event) = rx.recv().await {
        assert!(!matches!(event, AgentEvent::Completed { .. }));
    }
    server.await.unwrap();
}

#[tokio::test]
async fn cumulative_response_limit_stops_many_individually_small_events() {
    let content = "x".repeat(64 * 1024);
    let frame = format!("data: {{\"choices\":[{{\"delta\":{{\"content\":\"{content}\"}}}}]}}\n\n");
    let body = format!("{}data: [DONE]\n\n", frame.repeat(270));
    let (base, server) = serve(body.into_bytes(), false).await;
    let config = ModelConfig {
        api_base: Some(base),
        api_key: None,
        ..Default::default()
    };
    let (tx, mut rx) = mpsc::channel(300);
    let error = LlmClient::new()
        .stream_chat(&config, &[Message::user("test")], None, tx)
        .await
        .unwrap_err();
    assert!(error.to_string().contains("16 MiB"));
    while let Some(event) = rx.recv().await {
        assert!(!matches!(event, AgentEvent::Completed { .. }));
    }
    server.await.unwrap();
}

#[tokio::test]
async fn tool_identity_argument_and_count_limits_are_enforced() {
    for call in [
        serde_json::json!({"index":128,"id":"call","function":{"name":"calc","arguments":"{}"}}),
        serde_json::json!({"index":0,"id":"x".repeat(1025),"function":{"name":"calc","arguments":"{}"}}),
        serde_json::json!({"index":0,"id":"call","function":{"name":"calc","arguments":serde_json::json!({"content":"x".repeat(1024*1024)}).to_string()}}),
    ] {
        let event = serde_json::json!({"choices":[{"delta":{"tool_calls":[call]}}]});
        let body = format!("data: {event}\n\ndata: [DONE]\n\n");
        let (base, server) = serve(body.into_bytes(), false).await;
        let config = ModelConfig {
            api_base: Some(base),
            api_key: None,
            ..Default::default()
        };
        let (tx, mut rx) = mpsc::channel(16);
        let error = LlmClient::new()
            .stream_chat(&config, &[Message::user("test")], None, tx)
            .await
            .unwrap_err();
        assert!(error.to_string().contains("limit"));
        while let Some(event) = rx.recv().await {
            assert!(!matches!(event, AgentEvent::Completed { .. }));
        }
        server.await.unwrap();
    }
}

#[tokio::test]
async fn empty_or_usage_only_streams_cannot_report_a_completed_response() {
    for body in [
        "data: [DONE]\n\n",
        "data: {\"choices\":[],\"usage\":{\"total_tokens\":1}}\n\ndata: [DONE]\n\n",
        "data\n\ndata: [DONE]\n\n",
    ] {
        let (base, server) = serve(body.as_bytes().to_vec(), false).await;
        let config = ModelConfig {
            api_base: Some(base),
            api_key: None,
            ..Default::default()
        };
        let (tx, mut rx) = mpsc::channel(16);
        assert!(LlmClient::new()
            .stream_chat(&config, &[Message::user("test")], None, tx)
            .await
            .is_err());
        while let Some(event) = rx.recv().await {
            assert!(!matches!(event, AgentEvent::Completed { .. }));
        }
        server.await.unwrap();
    }
}

#[tokio::test]
async fn terminal_events_preserve_partial_text_without_accepting_new_output() {
    for (body, expected) in [
        ("data: {\"choices\":[{\"delta\":{\"content\":\"A\"},\"finish_reason\":\"stop\"}]}\n\ndata: {\"choices\":[{\"delta\":{\"content\":\"B\"}}]}\n\n", "A"),
        ("data: {\"choices\":[{\"delta\":{\"content\":\"最后一段\"},\"finish_reason\":\"length\"}]}\n\n", "最后一段"),
    ] {
        let (base, server) = serve(body.as_bytes().to_vec(), false).await;
        let config = ModelConfig {
            api_base: Some(base),
            api_key: None,
            ..Default::default()
        };
        let (tx, mut rx) = mpsc::channel(16);
        assert!(LlmClient::new()
            .stream_chat(&config, &[Message::user("test")], None, tx)
            .await
            .is_err());
        let mut partial = String::new();
        while let Some(event) = rx.recv().await {
            assert!(!matches!(event, AgentEvent::Completed { .. }));
            if let AgentEvent::TextDelta(text) = event {
                partial.push_str(&text);
            }
        }
        assert_eq!(partial, expected);
        server.await.unwrap();
    }
}
