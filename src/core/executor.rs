use crate::core::{Script, Step, AgentBrowser, ExecutionResult};
use crate::core::template::{render, RenderContext};
use crate::core::validation::validate_and_coerce;
use crate::core::pipeline::PipelineExecutor;
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
        mut params: HashMap<String, Value>,
    ) -> Result<ExecutionResult> {
        let start = Instant::now();
        let config = script.normalize();

        // Validate and coerce parameters
        validate_and_coerce(&config.params, &mut params)?;

        // Strategy pre-navigation
        if config.strategy.requires_pre_navigation() {
            if let Some(url) = config.strategy.pre_navigation_url(&config.domain) {
                let _ = self.browser.goto(&url).await;
                self.browser.wait(2000).await?;
            }
        }

        let mut context = HashMap::new();
        context.insert("args".to_string(), Value::Object(
            params.iter().map(|(k, v)| (k.clone(), v.clone())).collect()
        ));

        let result = if !config.steps.is_empty() {
            self.execute_steps(&config.steps, &params, &mut context).await?
        } else if let Some(pipeline) = &config.pipeline {
            let pipeline_exec = PipelineExecutor::new(&self.browser);
            pipeline_exec.execute(pipeline, &params).await?
        } else {
            Value::Null
        };

        Ok(ExecutionResult {
            execution_id: format!("exec_{}", chrono::Utc::now().timestamp()),
            status: "success".to_string(),
            duration_ms: start.elapsed().as_millis() as u64,
            result,
        })
    }

    async fn execute_steps(
        &self,
        steps: &[Step],
        params: &HashMap<String, Value>,
        context: &mut HashMap<String, Value>,
    ) -> Result<Value> {
        for step in steps {
            self.execute_step(step, params, context).await?;
        }
        Ok(context.get("result").cloned().unwrap_or(Value::Null))
    }

    async fn execute_step(
        &self,
        step: &Step,
        params: &HashMap<String, Value>,
        context: &mut HashMap<String, Value>,
    ) -> Result<()> {
        let render_ctx = RenderContext {
            args: params.clone(),
            data: context.get("data").cloned(),
            item: None,
            index: 0,
        };

        match step.action.as_str() {
            "navigate" => {
                if let Some(url) = &step.url {
                    let rendered = render(&Value::String(url.clone()), &render_ctx)?;
                    if let Value::String(url_str) = rendered {
                        self.browser.goto(&url_str).await?;
                    }
                }
            }
            "wait" => {
                let ms = step.duration.unwrap_or(1000);
                self.browser.wait(ms).await?;
            }
            "evaluate" => {
                if let Some(js) = &step.script {
                    let result = self.browser.eval(js).await?;
                    if let Some(output) = &step.output {
                        context.insert(output.clone(), result);
                    }
                }
            }
            "click" => {
                if let Some(sel) = &step.selector {
                    let rendered = render(&Value::String(sel.clone()), &render_ctx)?;
                    if let Value::String(selector) = rendered {
                        self.browser.click(&selector).await?;
                    }
                }
            }
            "fill" => {
                if let Some(sel) = &step.selector {
                    if let Some(val) = &step.value {
                        let rendered_sel = render(&Value::String(sel.clone()), &render_ctx)?;
                        let rendered_val = render(&Value::String(val.clone()), &render_ctx)?;
                        if let (Value::String(s), Value::String(v)) = (rendered_sel, rendered_val) {
                            self.browser.fill(&s, &v).await?;
                        }
                    }
                }
            }
            "type" => {
                if let Some(sel) = &step.selector {
                    if let Some(txt) = &step.text {
                        let rendered_sel = render(&Value::String(sel.clone()), &render_ctx)?;
                        let rendered_txt = render(&Value::String(txt.clone()), &render_ctx)?;
                        if let (Value::String(s), Value::String(t)) = (rendered_sel, rendered_txt) {
                            self.browser.type_text(&s, &t).await?;
                        }
                    }
                }
            }
            "press" => {
                if let Some(key) = &step.key {
                    self.browser.press(key).await?;
                }
            }
            "scroll" => {
                self.browser.scroll("down", "500").await?;
            }
            _ => {}
        }
        Ok(())
    }
}
