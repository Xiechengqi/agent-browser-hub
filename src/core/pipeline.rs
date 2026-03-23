use anyhow::Result;
use serde_json::Value;
use crate::core::AgentBrowser;
use crate::core::template::{render, RenderContext};
use std::collections::HashMap;

pub struct PipelineExecutor<'a> {
    browser: &'a AgentBrowser,
}

impl<'a> PipelineExecutor<'a> {
    pub fn new(browser: &'a AgentBrowser) -> Self {
        Self { browser }
    }

    pub async fn execute(
        &self,
        pipeline: &[Value],
        args: &HashMap<String, Value>,
    ) -> Result<Value> {
        let mut data = Value::Null;

        for step in pipeline {
            data = self.execute_step(step, args, &data).await?;
        }

        Ok(data)
    }

    async fn execute_step(
        &self,
        step: &Value,
        args: &HashMap<String, Value>,
        data: &Value,
    ) -> Result<Value> {
        let step_obj = step.as_object().ok_or_else(|| anyhow::anyhow!("Invalid step"))?;

        // Determine step type
        if let Some(step_type) = step_obj.keys().next() {
            match step_type.as_str() {
                "navigate" => self.step_navigate(step_obj, args, data).await,
                "evaluate" => self.step_evaluate(step_obj, args, data).await,
                "click" => self.step_click(step_obj, args, data).await,
                "type" => self.step_type(step_obj, args, data).await,
                "wait" => self.step_wait(step_obj, args, data).await,
                "press" => self.step_press(step_obj, args, data).await,
                "scroll" => self.step_scroll(step_obj, args, data).await,
                "snapshot" => self.step_snapshot(step_obj, args, data).await,
                "select" => self.step_select(step_obj, args, data),
                "map" => self.step_map(step_obj, args, data),
                "filter" => self.step_filter(step_obj, args, data),
                "sort" => self.step_sort(step_obj, args, data),
                "limit" => self.step_limit(step_obj, args, data),
                _ => Ok(data.clone()),
            }
        } else {
            Ok(data.clone())
        }
    }

    fn render_ctx(&self, args: &HashMap<String, Value>, data: &Value) -> RenderContext {
        RenderContext {
            args: args.clone(),
            data: Some(data.clone()),
            item: None,
            index: 0,
        }
    }

    async fn step_navigate(&self, step: &serde_json::Map<String, Value>, args: &HashMap<String, Value>, data: &Value) -> Result<Value> {
        if let Some(url) = step.get("navigate") {
            let ctx = self.render_ctx(args, data);
            let rendered = render(url, &ctx)?;
            if let Value::String(url_str) = rendered {
                self.browser.goto(&url_str).await?;
            }
        }
        Ok(data.clone())
    }

    async fn step_evaluate(&self, step: &serde_json::Map<String, Value>, _args: &HashMap<String, Value>, data: &Value) -> Result<Value> {
        if let Some(js) = step.get("evaluate").and_then(|v| v.as_str()) {
            let result = self.browser.eval(js).await?;
            return Ok(result);
        }
        Ok(data.clone())
    }

    async fn step_click(&self, step: &serde_json::Map<String, Value>, args: &HashMap<String, Value>, data: &Value) -> Result<Value> {
        if let Some(sel) = step.get("click") {
            let ctx = self.render_ctx(args, data);
            let rendered = render(sel, &ctx)?;
            if let Value::String(selector) = rendered {
                self.browser.click(&selector).await?;
            }
        }
        Ok(data.clone())
    }

    async fn step_type(&self, step: &serde_json::Map<String, Value>, args: &HashMap<String, Value>, data: &Value) -> Result<Value> {
        if let Some(type_obj) = step.get("type").and_then(|v| v.as_object()) {
            if let (Some(sel), Some(text)) = (type_obj.get("selector"), type_obj.get("text")) {
                let ctx = self.render_ctx(args, data);
                let rendered_sel = render(sel, &ctx)?;
                let rendered_text = render(text, &ctx)?;
                if let (Value::String(s), Value::String(t)) = (rendered_sel, rendered_text) {
                    self.browser.type_text(&s, &t).await?;
                }
            }
        }
        Ok(data.clone())
    }

    async fn step_wait(&self, step: &serde_json::Map<String, Value>, _args: &HashMap<String, Value>, data: &Value) -> Result<Value> {
        if let Some(wait_val) = step.get("wait") {
            if let Some(ms) = wait_val.as_u64() {
                self.browser.wait(ms).await?;
            } else if let Some(sel) = wait_val.as_str() {
                self.browser.wait_for_selector(sel).await?;
            }
        }
        Ok(data.clone())
    }

    async fn step_press(&self, step: &serde_json::Map<String, Value>, _args: &HashMap<String, Value>, data: &Value) -> Result<Value> {
        if let Some(key) = step.get("press").and_then(|v| v.as_str()) {
            self.browser.press(key).await?;
        }
        Ok(data.clone())
    }

    async fn step_scroll(&self, step: &serde_json::Map<String, Value>, _args: &HashMap<String, Value>, data: &Value) -> Result<Value> {
        if step.contains_key("scroll") {
            self.browser.scroll("down", "500").await?;
        }
        Ok(data.clone())
    }

    async fn step_snapshot(&self, step: &serde_json::Map<String, Value>, _args: &HashMap<String, Value>, _data: &Value) -> Result<Value> {
        if step.contains_key("snapshot") {
            return self.browser.snapshot().await;
        }
        Ok(Value::Null)
    }

    fn step_select(&self, step: &serde_json::Map<String, Value>, _args: &HashMap<String, Value>, data: &Value) -> Result<Value> {
        if let Some(path) = step.get("select").and_then(|v| v.as_str()) {
            let parts: Vec<&str> = path.split('.').collect();
            let mut current = data.clone();
            for part in parts {
                current = match &current {
                    Value::Object(map) => map.get(part).cloned().unwrap_or(Value::Null),
                    Value::Array(arr) => {
                        if let Ok(idx) = part.parse::<usize>() {
                            arr.get(idx).cloned().unwrap_or(Value::Null)
                        } else {
                            Value::Null
                        }
                    }
                    _ => Value::Null,
                };
            }
            return Ok(current);
        }
        Ok(data.clone())
    }

    fn step_map(&self, step: &serde_json::Map<String, Value>, args: &HashMap<String, Value>, data: &Value) -> Result<Value> {
        if let Some(map_obj) = step.get("map").and_then(|v| v.as_object()) {
            if let Value::Array(arr) = data {
                let mut results = Vec::new();
                for (idx, item) in arr.iter().enumerate() {
                    let ctx = RenderContext {
                        args: args.clone(),
                        data: Some(data.clone()),
                        item: Some(item.clone()),
                        index: idx,
                    };
                    let mut mapped = serde_json::Map::new();
                    for (key, template) in map_obj {
                        let rendered = render(template, &ctx)?;
                        mapped.insert(key.clone(), rendered);
                    }
                    results.push(Value::Object(mapped));
                }
                return Ok(Value::Array(results));
            }
        }
        Ok(data.clone())
    }

    fn step_filter(&self, step: &serde_json::Map<String, Value>, _args: &HashMap<String, Value>, data: &Value) -> Result<Value> {
        if let Some(_filter_expr) = step.get("filter") {
            if let Value::Array(arr) = data {
                // Simple filter: keep non-null items
                let filtered: Vec<Value> = arr.iter().filter(|v| !v.is_null()).cloned().collect();
                return Ok(Value::Array(filtered));
            }
        }
        Ok(data.clone())
    }

    fn step_sort(&self, step: &serde_json::Map<String, Value>, _args: &HashMap<String, Value>, data: &Value) -> Result<Value> {
        if let Some(sort_key) = step.get("sort").and_then(|v| v.as_str()) {
            if let Value::Array(mut arr) = data.clone() {
                arr.sort_by(|a, b| {
                    let a_val = a.get(sort_key).and_then(|v| v.as_str()).unwrap_or("");
                    let b_val = b.get(sort_key).and_then(|v| v.as_str()).unwrap_or("");
                    a_val.cmp(b_val)
                });
                return Ok(Value::Array(arr));
            }
        }
        Ok(data.clone())
    }

    fn step_limit(&self, step: &serde_json::Map<String, Value>, _args: &HashMap<String, Value>, data: &Value) -> Result<Value> {
        if let Some(n) = step.get("limit").and_then(|v| v.as_u64()) {
            if let Value::Array(arr) = data {
                let limited: Vec<Value> = arr.iter().take(n as usize).cloned().collect();
                return Ok(Value::Array(limited));
            }
        }
        Ok(data.clone())
    }
}


