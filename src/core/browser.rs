use anyhow::Result;
use serde_json::Value;
use std::process::{Child, Command, Stdio};

pub struct AgentBrowser {
    process: Option<Child>,
}

impl AgentBrowser {
    pub async fn new() -> Result<Self> {
        Ok(Self { process: None })
    }

    pub async fn execute(&self, cmd: &str) -> Result<String> {
        let output = Command::new("agent-browser")
            .args(cmd.split_whitespace())
            .output()?;

        Ok(String::from_utf8(output.stdout)?)
    }

    pub async fn goto(&self, url: &str) -> Result<()> {
        self.execute(&format!("open {}", url)).await?;
        Ok(())
    }

    pub async fn eval(&self, js: &str) -> Result<Value> {
        let result = self.execute(&format!("eval '{}'", js)).await?;
        Ok(serde_json::from_str(&result)?)
    }

    pub async fn wait(&self, seconds: u64) -> Result<()> {
        tokio::time::sleep(tokio::time::Duration::from_secs(seconds)).await;
        Ok(())
    }
}

impl Drop for AgentBrowser {
    fn drop(&mut self) {
        if let Some(mut child) = self.process.take() {
            let _ = child.kill();
        }
    }
}
