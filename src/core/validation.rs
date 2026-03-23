use anyhow::Result;
use serde_json::Value;
use std::collections::HashMap;

use crate::core::ParamDef;

pub fn validate_and_coerce(
    params: &[ParamDef],
    args: &mut HashMap<String, Value>,
) -> Result<()> {
    for param in params {
        let val = args.get(&param.name);

        // Check required
        if param.required && (val.is_none() || val == Some(&Value::Null)) {
            anyhow::bail!("Parameter '{}' is required", param.name);
        }

        // Apply default
        if val.is_none() {
            if let Some(default) = &param.default {
                args.insert(param.name.clone(), default.clone());
            }
            continue;
        }

        let val = val.unwrap();

        // Type coercion
        let coerced = match param.type_.as_str() {
            "integer" | "int" => {
                if let Some(n) = val.as_i64() {
                    Value::Number(n.into())
                } else if let Some(s) = val.as_str() {
                    Value::Number(s.parse::<i64>()?.into())
                } else {
                    val.clone()
                }
            }
            "number" | "float" => {
                if let Some(n) = val.as_f64() {
                    Value::Number(serde_json::Number::from_f64(n).unwrap())
                } else if let Some(s) = val.as_str() {
                    let n = s.parse::<f64>()?;
                    Value::Number(serde_json::Number::from_f64(n).unwrap())
                } else {
                    val.clone()
                }
            }
            "boolean" | "bool" => {
                if let Some(b) = val.as_bool() {
                    Value::Bool(b)
                } else if let Some(s) = val.as_str() {
                    Value::Bool(matches!(s.to_lowercase().as_str(), "true" | "1" | "yes"))
                } else {
                    val.clone()
                }
            }
            _ => val.clone(),
        };

        args.insert(param.name.clone(), coerced);
    }

    Ok(())
}
