use anyhow::Result;
use serde_json::Value;
use tokio::process::Command;

pub struct AgentBrowser {
    agent_browser_path: String,
}

impl AgentBrowser {
    pub async fn new() -> Result<Self> {
        Ok(Self {
            agent_browser_path: "/data/projects/agent-browser/bin/agent-browser.js".to_string(),
        })
    }

    async fn run_command(&self, args: &[&str]) -> Result<String> {
        let mut cmd = Command::new("node");
        cmd.arg(&self.agent_browser_path);
        for arg in args {
            cmd.arg(arg);
        }
        let output = cmd.output().await?;
        if !output.status.success() {
            anyhow::bail!("{}", String::from_utf8_lossy(&output.stderr));
        }
        Ok(String::from_utf8(output.stdout)?)
    }

    async fn run_command_json(&self, args: &[&str]) -> Result<Value> {
        let stdout = self.run_command(args).await?;
        Ok(serde_json::from_str(&stdout)?)
    }

    // Navigation
    pub async fn goto(&self, url: &str) -> Result<()> {
        self.run_command(&["open", url]).await?;
        Ok(())
    }

    // Execution
    pub async fn eval(&self, js: &str) -> Result<Value> {
        self.run_command_json(&["eval", js, "--json"]).await
    }

    pub async fn eval_base64(&self, js_b64: &str) -> Result<Value> {
        self.run_command_json(&["eval", js_b64, "-b", "--json"]).await
    }

    // Wait operations
    pub async fn wait(&self, ms: u64) -> Result<()> {
        self.run_command(&["wait", &ms.to_string()]).await?;
        Ok(())
    }

    pub async fn wait_for_selector(&self, selector: &str) -> Result<()> {
        self.run_command(&["wait", selector]).await?;
        Ok(())
    }

    pub async fn wait_for_text(&self, text: &str) -> Result<()> {
        self.run_command(&["wait", "--text", text]).await?;
        Ok(())
    }

    pub async fn wait_for_url(&self, pattern: &str) -> Result<()> {
        self.run_command(&["wait", "--url", pattern]).await?;
        Ok(())
    }

    // DOM interactions
    pub async fn click(&self, selector: &str) -> Result<()> {
        self.run_command(&["click", selector]).await?;
        Ok(())
    }

    pub async fn fill(&self, selector: &str, value: &str) -> Result<()> {
        self.run_command(&["fill", selector, value]).await?;
        Ok(())
    }

    pub async fn type_text(&self, selector: &str, text: &str) -> Result<()> {
        self.run_command(&["type", selector, text]).await?;
        Ok(())
    }

    pub async fn press(&self, key: &str) -> Result<()> {
        self.run_command(&["press", key]).await?;
        Ok(())
    }

    pub async fn hover(&self, selector: &str) -> Result<()> {
        self.run_command(&["hover", selector]).await?;
        Ok(())
    }

    pub async fn scroll(&self, direction: &str, amount: &str) -> Result<()> {
        self.run_command(&["scroll", direction, amount]).await?;
        Ok(())
    }

    pub async fn scroll_into_view(&self, selector: &str) -> Result<()> {
        let js = format!("document.querySelector('{}').scrollIntoView()", selector);
        self.eval(&js).await?;
        Ok(())
    }

    // Element queries
    pub async fn get_text(&self, selector: &str) -> Result<String> {
        let js = format!("document.querySelector('{}')?.textContent || ''", selector);
        let result = self.eval(&js).await?;
        Ok(result.as_str().unwrap_or("").to_string())
    }

    pub async fn get_html(&self, selector: &str) -> Result<String> {
        let js = format!("document.querySelector('{}')?.innerHTML || ''", selector);
        let result = self.eval(&js).await?;
        Ok(result.as_str().unwrap_or("").to_string())
    }

    pub async fn get_value(&self, selector: &str) -> Result<String> {
        let js = format!("document.querySelector('{}')?.value || ''", selector);
        let result = self.eval(&js).await?;
        Ok(result.as_str().unwrap_or("").to_string())
    }

    pub async fn get_attr(&self, selector: &str, attr: &str) -> Result<String> {
        let js = format!("document.querySelector('{}')?.getAttribute('{}') || ''", selector, attr);
        let result = self.eval(&js).await?;
        Ok(result.as_str().unwrap_or("").to_string())
    }

    pub async fn get_count(&self, selector: &str) -> Result<usize> {
        let js = format!("document.querySelectorAll('{}').length", selector);
        let result = self.eval(&js).await?;
        Ok(result.as_u64().unwrap_or(0) as usize)
    }

    pub async fn is_visible(&self, selector: &str) -> Result<bool> {
        let js = format!(
            "(() => {{ const el = document.querySelector('{}'); return el && el.offsetParent !== null; }})()",
            selector
        );
        let result = self.eval(&js).await?;
        Ok(result.as_bool().unwrap_or(false))
    }

    // Cookies
    pub async fn get_cookies(&self) -> Result<Value> {
        self.run_command_json(&["cookies"]).await
    }

    pub async fn set_cookie(&self, name: &str, value: &str, domain: Option<&str>) -> Result<()> {
        let mut args = vec!["cookies", "set", name, value];
        if let Some(d) = domain {
            args.push("--domain");
            args.push(d);
        }
        self.run_command(&args).await?;
        Ok(())
    }

    pub async fn clear_cookies(&self) -> Result<()> {
        self.run_command(&["cookies", "clear"]).await?;
        Ok(())
    }

    // Network
    pub async fn network_route(&self, url: &str, handler: &str) -> Result<()> {
        self.run_command(&["network", "route", url, handler]).await?;
        Ok(())
    }

    pub async fn network_unroute(&self, url: &str) -> Result<()> {
        self.run_command(&["network", "unroute", url]).await?;
        Ok(())
    }

    pub async fn network_requests(&self, filter: Option<&str>) -> Result<Value> {
        let mut args = vec!["network", "requests"];
        if let Some(f) = filter {
            args.push(f);
        }
        self.run_command_json(&args).await
    }

    pub async fn set_headers(&self, headers: &str) -> Result<()> {
        self.run_command(&["headers", headers]).await?;
        Ok(())
    }

    // Tabs
    pub async fn list_tabs(&self) -> Result<Value> {
        self.run_command_json(&["tab"]).await
    }

    pub async fn new_tab(&self, url: Option<&str>) -> Result<()> {
        let mut args = vec!["tab", "new"];
        if let Some(u) = url {
            args.push(u);
        }
        self.run_command(&args).await?;
        Ok(())
    }

    pub async fn switch_tab(&self, n: usize) -> Result<()> {
        self.run_command(&["tab", &n.to_string()]).await?;
        Ok(())
    }

    pub async fn close_tab(&self, n: usize) -> Result<()> {
        self.run_command(&["tab", "close", &n.to_string()]).await?;
        Ok(())
    }

    // Screenshot & Snapshot
    pub async fn screenshot(&self, path: Option<&str>) -> Result<()> {
        let mut args = vec!["screenshot"];
        if let Some(p) = path {
            args.push(p);
        }
        self.run_command(&args).await?;
        Ok(())
    }

    pub async fn snapshot(&self) -> Result<Value> {
        self.run_command_json(&["snapshot", "--json"]).await
    }

    // State
    pub async fn save_state(&self, path: &str) -> Result<()> {
        self.run_command(&["state", "save", path]).await?;
        Ok(())
    }

    pub async fn load_state(&self, path: &str) -> Result<()> {
        self.run_command(&["state", "load", path]).await?;
        Ok(())
    }

    // Advanced
    pub async fn install_interceptor(&self, js: &str) -> Result<()> {
        self.eval(js).await?;
        Ok(())
    }

    pub async fn get_intercepted_requests(&self, array_name: &str) -> Result<Value> {
        let js = format!(
            "(() => {{ const data = window.{} || []; window.{} = []; return data; }})()",
            array_name, array_name
        );
        self.eval(&js).await
    }

    pub async fn auto_scroll(&self, times: usize, delay_ms: u64) -> Result<()> {
        for _ in 0..times {
            self.scroll("down", "500").await?;
            self.wait(delay_ms).await?;
        }
        Ok(())
    }
}
