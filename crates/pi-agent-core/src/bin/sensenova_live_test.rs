use anyhow::{ensure, Context as _, Result};
use pi_agent_core::{AgentEvent, Message, ModelConfig, PiAgent};
use tokio::sync::mpsc;

const EXPECTED_OUTPUT: &str = "SenseNova-Volt-Integration-Success";

#[tokio::main]
async fn main() -> Result<()> {
    println!("=== Testing SenseNova Live Agent Integration ===");

    let api_key = ["SENSENOVA_API_KEY", "OPENAI_API_KEY"]
        .into_iter()
        .filter_map(|name| std::env::var(name).ok())
        .find(|key| !key.trim().is_empty())
        .context("Live test requires SENSENOVA_API_KEY or OPENAI_API_KEY")?;

    let config = ModelConfig {
        provider: "openai-compatible".to_string(),
        model: "sensenova-6.8-flash-lite".to_string(),
        api_base: Some(
            std::env::var("OPENAI_BASE_URL")
                .unwrap_or_else(|_| "https://token.sensenova.cn/v1/chat/completions".into()),
        ),
        api_key: Some(api_key),
        temperature: 0.1,
        max_tokens: Some(1024),
    };

    let workspace = std::env::current_dir()?;
    let agent = PiAgent::new(workspace, config);

    let mut messages = vec![Message::user(
        "请使用 bash 工具执行命令 `echo 'SenseNova-Volt-Integration-Success'` 并输出结果。",
    )];

    let (tx, mut rx) = mpsc::channel::<AgentEvent>(100);

    let agent_handle = tokio::spawn(async move { agent.run_turn(&mut messages, tx).await });
    let mut verified_tool_output = false;
    let mut tool_failed = false;
    let mut provider_failed = false;
    let mut model_completed = false;

    while let Some(event) = rx.recv().await {
        match event {
            AgentEvent::WorkflowUpdated(snapshot) => {
                println!("[WORKFLOW] {} {:?}", snapshot.id, snapshot.status);
            }
            AgentEvent::Thought(thought) => {
                print!("[THOUGHT] {}", thought);
            }
            AgentEvent::TextDelta(delta) => {
                print!("{}", delta);
            }
            AgentEvent::ToolCallStarted {
                id,
                name,
                arguments,
            } => {
                println!(
                    "\n>>> [TOOL CALL START] {} (id: {}): {}",
                    name, id, arguments
                );
            }
            AgentEvent::ToolCallFinished {
                id,
                name,
                output,
                is_error,
            } => {
                tool_failed |= is_error;
                verified_tool_output |=
                    !is_error && name == "bash" && output.trim() == EXPECTED_OUTPUT;
                println!(
                    "<<< [TOOL CALL FINISHED] {} (id: {}, err: {}):\n{}",
                    name, id, is_error, output
                );
            }
            AgentEvent::Completed { total_tokens } => {
                model_completed = true;
                println!("\n[COMPLETED] Total tokens: {:?}", total_tokens);
            }
            AgentEvent::Error(err) => {
                provider_failed = true;
                eprintln!("\n[ERROR] {}", err);
            }
        }
    }

    agent_handle.await??;
    ensure!(
        !tool_failed && !provider_failed,
        "Live test reported a failure"
    );
    ensure!(
        model_completed,
        "Live test did not complete a model response"
    );
    ensure!(
        verified_tool_output,
        "Live test did not execute bash successfully with the exact requested output"
    );
    println!("\n=== SenseNova Live Agent Integration Test PASSED! ===");
    Ok(())
}
