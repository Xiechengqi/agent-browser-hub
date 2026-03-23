use anyhow::Result;
use comfy_table::{Table, presets::UTF8_FULL};
use serde_json::Value;

#[derive(Debug, Clone)]
pub enum OutputFormat {
    Json,
    Yaml,
    Table,
    Csv,
    Markdown,
}

impl OutputFormat {
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "yaml" | "yml" => OutputFormat::Yaml,
            "table" => OutputFormat::Table,
            "csv" => OutputFormat::Csv,
            "md" | "markdown" => OutputFormat::Markdown,
            _ => OutputFormat::Json,
        }
    }
}

pub fn format_output(data: &Value, format: &OutputFormat) -> Result<String> {
    match format {
        OutputFormat::Json => Ok(serde_json::to_string_pretty(data)?),
        OutputFormat::Yaml => Ok(serde_yaml::to_string(data)?),
        OutputFormat::Table => format_table(data),
        OutputFormat::Csv => format_csv(data),
        OutputFormat::Markdown => format_markdown(data),
    }
}

fn format_table(data: &Value) -> Result<String> {
    let arr = match data {
        Value::Array(a) => a,
        _ => return Ok(serde_json::to_string_pretty(data)?),
    };

    if arr.is_empty() {
        return Ok("(empty)".to_string());
    }

    let mut table = Table::new();
    table.load_preset(UTF8_FULL);

    // Extract headers from first object
    if let Some(Value::Object(first)) = arr.first() {
        let headers: Vec<String> = first.keys().cloned().collect();
        table.set_header(&headers);

        for item in arr {
            if let Value::Object(obj) = item {
                let row: Vec<String> = headers.iter()
                    .map(|h| value_to_string(obj.get(h).unwrap_or(&Value::Null)))
                    .collect();
                table.add_row(row);
            }
        }
    }

    Ok(table.to_string())
}

fn format_csv(data: &Value) -> Result<String> {
    let arr = match data {
        Value::Array(a) => a,
        _ => return Ok(serde_json::to_string(data)?),
    };

    if arr.is_empty() {
        return Ok(String::new());
    }

    let mut output = String::new();

    if let Some(Value::Object(first)) = arr.first() {
        let headers: Vec<String> = first.keys().cloned().collect();
        output.push_str(&headers.join(","));
        output.push('\n');

        for item in arr {
            if let Value::Object(obj) = item {
                let row: Vec<String> = headers.iter()
                    .map(|h| csv_escape(&value_to_string(obj.get(h).unwrap_or(&Value::Null))))
                    .collect();
                output.push_str(&row.join(","));
                output.push('\n');
            }
        }
    }

    Ok(output)
}

fn format_markdown(data: &Value) -> Result<String> {
    let arr = match data {
        Value::Array(a) => a,
        _ => return Ok(format!("```json\n{}\n```", serde_json::to_string_pretty(data)?)),
    };

    if arr.is_empty() {
        return Ok("(empty)".to_string());
    }

    let mut output = String::new();

    if let Some(Value::Object(first)) = arr.first() {
        let headers: Vec<String> = first.keys().cloned().collect();

        output.push_str("| ");
        output.push_str(&headers.join(" | "));
        output.push_str(" |\n");

        output.push_str("|");
        for _ in &headers {
            output.push_str(" --- |");
        }
        output.push('\n');

        for item in arr {
            if let Value::Object(obj) = item {
                output.push_str("| ");
                let row: Vec<String> = headers.iter()
                    .map(|h| value_to_string(obj.get(h).unwrap_or(&Value::Null)))
                    .collect();
                output.push_str(&row.join(" | "));
                output.push_str(" |\n");
            }
        }
    }

    Ok(output)
}

fn value_to_string(v: &Value) -> String {
    match v {
        Value::String(s) => s.clone(),
        Value::Number(n) => n.to_string(),
        Value::Bool(b) => b.to_string(),
        Value::Null => String::new(),
        _ => serde_json::to_string(v).unwrap_or_default(),
    }
}

fn csv_escape(s: &str) -> String {
    if s.contains(',') || s.contains('"') || s.contains('\n') {
        format!("\"{}\"", s.replace('"', "\"\""))
    } else {
        s.to_string()
    }
}
