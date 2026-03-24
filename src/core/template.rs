use anyhow::Result;
use regex::Regex;
use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct RenderContext {
    pub args: HashMap<String, Value>,
    pub data: Option<Value>,
    pub item: Option<Value>,
    pub index: usize,
}

impl RenderContext {
    pub fn new() -> Self {
        Self {
            args: HashMap::new(),
            data: None,
            item: None,
            index: 0,
        }
    }
}

pub fn render(template: &Value, ctx: &RenderContext) -> Result<Value> {
    match template {
        Value::String(s) => {
            let trimmed = s.trim();

            // Full expression: ${{ expr }}
            if let Some(caps) = Regex::new(r"^\$\{\{\s*(.+?)\s*\}\}$")?.captures(trimmed) {
                if !trimmed.contains("}}-") && !trimmed.contains("}}${{") {
                    return eval_expr(&caps[1], ctx);
                }
            }

            // Replace inline expressions
            let re = Regex::new(r"\$\{\{\s*(.+?)\s*\}\}")?;
            let result = re.replace_all(s, |caps: &regex::Captures| {
                eval_expr(&caps[1], ctx)
                    .and_then(|v| Ok(value_to_string(&v)))
                    .unwrap_or_else(|_| caps[0].to_string())
            });

            // Also support {{key}} syntax
            let re2 = Regex::new(r"\{\{(\w+)\}\}")?;
            let result = re2.replace_all(&result, |caps: &regex::Captures| {
                ctx.args
                    .get(&caps[1])
                    .map(|v| value_to_string(v))
                    .unwrap_or_else(|| caps[0].to_string())
            });

            Ok(Value::String(result.to_string()))
        }
        _ => Ok(template.clone()),
    }
}

fn eval_expr(expr: &str, ctx: &RenderContext) -> Result<Value> {
    let expr = expr.trim();

    // Pipe filters: expr | filter1 | filter2
    if expr.contains('|') && !expr.contains("||") {
        let parts: Vec<&str> = expr.split('|').map(|s| s.trim()).collect();
        let mut result = resolve_path(parts[0], ctx)?;
        for filter in &parts[1..] {
            result = apply_filter(filter, &result)?;
        }
        return Ok(result);
    }

    // Arithmetic: index + 1
    if let Some(caps) = Regex::new(r"^(\w+(?:\.\w+)*)\s*([+\-*/])\s*(\d+)$")?.captures(expr) {
        let val = resolve_path(&caps[1], ctx)?;
        if let Some(num_val) = val.as_f64() {
            let operand = caps[3].parse::<f64>()?;
            let result = match &caps[2] {
                "+" => num_val + operand,
                "-" => num_val - operand,
                "*" => num_val * operand,
                "/" => {
                    if operand != 0.0 {
                        num_val / operand
                    } else {
                        0.0
                    }
                }
                _ => num_val,
            };
            return Ok(Value::Number(serde_json::Number::from_f64(result).unwrap()));
        }
    }

    // OR expression: item.count || 'N/A'
    if let Some(caps) = Regex::new(r"^(.+?)\s*\|\|\s*(.+)$")?.captures(expr) {
        let left = eval_expr(caps[1].trim(), ctx)?;
        if !is_falsy(&left) {
            return Ok(left);
        }
        let right = caps[2].trim().trim_matches(|c| c == '\'' || c == '"');
        return Ok(Value::String(right.to_string()));
    }

    resolve_path(expr, ctx)
}

fn resolve_path(path: &str, ctx: &RenderContext) -> Result<Value> {
    let normalized = normalize_bracket_path(path);
    let parts: Vec<&str> = normalized
        .split('.')
        .filter(|part| !part.is_empty())
        .collect();
    if parts.is_empty() {
        return Ok(Value::Null);
    }
    let root = parts[0];

    let mut obj = match root {
        "args" => Value::Object(
            ctx.args
                .iter()
                .map(|(k, v)| (k.clone(), v.clone()))
                .collect(),
        ),
        "data" => ctx.data.clone().unwrap_or(Value::Null),
        "item" => ctx.item.clone().unwrap_or(Value::Null),
        "index" => Value::Number(ctx.index.into()),
        _ => {
            // Default to item namespace
            if let Some(item) = &ctx.item {
                item.clone()
            } else {
                ctx.args.get(root).cloned().unwrap_or(Value::Null)
            }
        }
    };

    let rest = if root == "args" || root == "data" || root == "item" || root == "index" {
        &parts[1..]
    } else {
        &parts[..]
    };

    for part in rest {
        obj = match &obj {
            Value::Object(map) => map.get(*part).cloned().unwrap_or(Value::Null),
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

    Ok(obj)
}

fn normalize_bracket_path(path: &str) -> String {
    let mut out = String::with_capacity(path.len());
    let chars: Vec<char> = path.chars().collect();
    let mut i = 0;

    while i < chars.len() {
        if chars[i] == '[' {
            let quote = chars.get(i + 1).copied();
            if matches!(quote, Some('\'') | Some('"')) {
                let mut j = i + 2;
                while j < chars.len() && chars[j] != quote.unwrap() {
                    j += 1;
                }
                if j + 1 < chars.len() && chars[j + 1] == ']' {
                    out.push('.');
                    for ch in &chars[i + 2..j] {
                        out.push(*ch);
                    }
                    i = j + 2;
                    continue;
                }
            }
        }
        out.push(chars[i]);
        i += 1;
    }

    out
}

fn apply_filter(filter_expr: &str, value: &Value) -> Result<Value> {
    let re = Regex::new(r"^(\w+)(?:\((.+?)\))?$")?;
    let caps = re
        .captures(filter_expr)
        .ok_or_else(|| anyhow::anyhow!("Invalid filter"))?;
    let name = &caps[1];
    let arg = caps
        .get(2)
        .map(|m| m.as_str().trim_matches(|c| c == '\'' || c == '"'));

    match name {
        "default" => {
            if is_falsy(value) {
                Ok(Value::String(arg.unwrap_or("").to_string()))
            } else {
                Ok(value.clone())
            }
        }
        "join" => {
            if let Value::Array(arr) = value {
                let sep = arg.unwrap_or(", ");
                let joined = arr
                    .iter()
                    .map(|v| value_to_string(v))
                    .collect::<Vec<_>>()
                    .join(sep);
                Ok(Value::String(joined))
            } else {
                Ok(value.clone())
            }
        }
        "upper" => Ok(Value::String(value_to_string(value).to_uppercase())),
        "lower" => Ok(Value::String(value_to_string(value).to_lowercase())),
        "trim" => Ok(Value::String(value_to_string(value).trim().to_string())),
        "truncate" => {
            let n = arg.and_then(|a| a.parse::<usize>().ok()).unwrap_or(50);
            let s = value_to_string(value);
            if s.len() > n {
                Ok(Value::String(format!("{}...", &s[..n])))
            } else {
                Ok(Value::String(s))
            }
        }
        "replace" => {
            if let Some(args) = arg {
                let parts: Vec<&str> = args
                    .split(',')
                    .map(|s| s.trim().trim_matches(|c| c == '\'' || c == '"'))
                    .collect();
                if parts.len() >= 2 {
                    let s = value_to_string(value);
                    Ok(Value::String(s.replace(parts[0], parts[1])))
                } else {
                    Ok(value.clone())
                }
            } else {
                Ok(value.clone())
            }
        }
        "length" => {
            let len = match value {
                Value::Array(arr) => arr.len(),
                Value::String(s) => s.len(),
                _ => 0,
            };
            Ok(Value::Number(len.into()))
        }
        "first" => {
            if let Value::Array(arr) = value {
                Ok(arr.first().cloned().unwrap_or(Value::Null))
            } else {
                Ok(value.clone())
            }
        }
        "last" => {
            if let Value::Array(arr) = value {
                Ok(arr.last().cloned().unwrap_or(Value::Null))
            } else {
                Ok(value.clone())
            }
        }
        "json" => Ok(Value::String(serde_json::to_string(value)?)),
        "urlencode" => Ok(Value::String(
            urlencoding::encode(&value_to_string(value)).to_string(),
        )),
        _ => Ok(value.clone()),
    }
}

fn is_falsy(value: &Value) -> bool {
    match value {
        Value::Null => true,
        Value::Bool(b) => !b,
        Value::String(s) => s.is_empty(),
        Value::Number(n) => n.as_f64().map(|f| f == 0.0).unwrap_or(false),
        Value::Array(a) => a.is_empty(),
        _ => false,
    }
}

fn value_to_string(value: &Value) -> String {
    match value {
        Value::String(s) => s.clone(),
        Value::Number(n) => n.to_string(),
        Value::Bool(b) => b.to_string(),
        Value::Null => String::new(),
        _ => serde_json::to_string(value).unwrap_or_default(),
    }
}
