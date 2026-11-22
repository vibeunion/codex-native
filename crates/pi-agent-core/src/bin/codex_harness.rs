use anyhow::Result;
use pi_agent_core::{CodexHarnessServer, ModelConfig};

#[tokio::main]
async fn main() -> Result<()> {
    let workspace = std::env::var_os("VOLT_WORKSPACE")
        .map(std::path::PathBuf::from)
        .unwrap_or(std::env::current_dir()?);
    CodexHarnessServer::new(workspace, ModelConfig::default())
        .run_stdio()
        .await
}
