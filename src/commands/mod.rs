pub mod bilibili;

use crate::registry::Registry;
use anyhow::Result;
use serde_json::Value;
use std::collections::HashMap;

pub fn register_all(registry: &mut Registry) {
    bilibili::register(registry);
}

pub async fn dispatch_native(handler: &str, params: HashMap<String, Value>) -> Result<Value> {
    match handler {
        "bilibili_feed" => bilibili::dispatch_native(handler, params).await,
        _ => anyhow::bail!("unknown native handler: {}", handler),
    }
}
