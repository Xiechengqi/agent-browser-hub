use crate::core::{Script, Step, AgentBrowser, ExecutionResult};
use anyhow::Result;
use serde_json::Value;
use std::collections::HashMap;
use std::time::Instant;

pub struct Executor {
    browser: AgentBrowser,
}

impl Executor {
    pub async fn new() -> Result<Self> {
        Ok(Self {
            browser: AgentBrowser::new().await?,
        })
    }

    pub async fn execute(
        &self,
        script: &Script,
        params: HashMap<String, Value>,
    ) -> Result<ExecutionResult> {
        let start = Instant::now();
        let mut context = HashMap::new();

        for step in &script.steps {
            self.execute_step(step, &params, &mut context).await?;
        }

        let result = context.get("result").cloned().unwrap_or(Value::Null);

        Ok(ExecutionResult {
            execution_id: format!("exec_{}", chrono::Utc::now().timestamp()),
            status: "success".to_string(),
            duration_ms: start.elapsed().as_millis() as u64,
            result,
        })
    }

    async fn execute_step(
        &self,
        step: &Step,
        params: &HashMap<String, Value>,
        context: &mut HashMap<String, Value>,
    ) -> Result<()> {
        match step.action.as_str() {
            "navigate" => {
                let url = self.interpolate(&step.url.as_ref().unwrap(), params)?;
                self.browser.goto(&url).await?;
            }
            "wait" => {
                self.browser.wait(step.duration.unwrap_or(1)).await?;
            }
            "evaluate" => {
                let js = step.script.as_ref().unwrap();
                let result = self.browser.eval(js).await?;
                if let Some(output) = &step.output {
                    context.insert(output.clone(), result);
                }
            }
            _ => {}
        }
        Ok(())
    }

    fn interpolate(&self, template: &str, params: &HashMap<String, Value>) -> Result<String> {
        let mut result = template.to_string();
        for (key, value) in params {
            let placeholder = format!("{{{{{}}}}}", key);
            let value_str = match value {
                Value::String(s) => s.clone(),
                _ => value.to_string(),
            };
            result = result.replace(&placeholder, &value_str);
        }
        Ok(result)
    }
}
