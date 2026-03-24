pub mod utils;

use crate::core::AgentBrowser;
use crate::registry::{CommandEntry, CommandSource, Registry};
use anyhow::Result;
use serde_json::Value;
use std::collections::HashMap;

pub fn register(registry: &mut Registry) {
    registry.register(CommandEntry {
        site: "bilibili".to_string(),
        name: "feed".to_string(),
        description: "Get bilibili feed with WBI signature".to_string(),
        strategy: Some("COOKIE".to_string()),
        source_label: "native".to_string(),
        source: CommandSource::Native("bilibili_feed".to_string()),
    });
}

pub async fn dispatch_native(handler: &str, params: HashMap<String, Value>) -> Result<Value> {
    match handler {
        "bilibili_feed" => execute_feed(params).await,
        _ => anyhow::bail!("unsupported bilibili native handler: {}", handler),
    }
}

async fn execute_feed(params: HashMap<String, Value>) -> Result<Value> {
    let limit = param_as_u64(&params, "limit").unwrap_or(20).min(100);
    let filter_type = param_as_string(&params, "type").unwrap_or_else(|| "all".to_string());
    let filter_type = match filter_type.as_str() {
        "video" => "video",
        "article" => "article",
        _ => "all",
    };

    let browser = AgentBrowser::new().await?;
    browser.goto("https://www.bilibili.com").await?;
    browser.wait(1500).await?;

    let js = format!(
        r#"(async () => {{
            const params = new URLSearchParams({{
              timezone_offset: '-480',
              type: {filter_type:?},
              page: '1',
            }});
            const res = await fetch('https://api.bilibili.com/x/polymer/web-dynamic/v1/feed/all?' + params.toString(), {{
              credentials: 'include'
            }});
            const payload = await res.json();
            const items = payload?.data?.items || [];
            const rows = [];

            function stripHtml(s) {{
              return String(s || '').replace(/<[^>]+>/g, '').replace(/&[a-z]+;/gi, ' ').trim();
            }}

            for (let i = 0; i < items.length; i++) {{
              const item = items[i];
              const modules = item.modules || {{}};
              const authorModule = modules.module_author || {{}};
              const dynamicModule = modules.module_dynamic || {{}};
              const major = dynamicModule.major || {{}};

              let title = '';
              let url = '';
              let itemType = item.type || '';

              if (major.archive) {{
                title = major.archive.title || '';
                url = major.archive.jump_url ? 'https:' + major.archive.jump_url : '';
                itemType = 'video';
              }} else if (major.article) {{
                title = major.article.title || '';
                url = major.article.jump_url ? 'https:' + major.article.jump_url : '';
                itemType = 'article';
              }} else if (dynamicModule.desc) {{
                title = stripHtml(dynamicModule.desc.text || '').slice(0, 60);
                url = item.id_str ? 'https://t.bilibili.com/' + item.id_str : '';
                itemType = 'dynamic';
              }}

              if (!title) continue;

              rows.push({{
                rank: rows.length + 1,
                author: authorModule.name || '',
                title,
                type: itemType,
                url,
              }});
            }}

            return rows.slice(0, {limit});
        }})()"#,
        filter_type = filter_type,
        limit = limit,
    );

    browser.eval(&js).await
}

fn param_as_u64(params: &HashMap<String, Value>, key: &str) -> Option<u64> {
    params.get(key).and_then(|value| match value {
        Value::Number(num) => num.as_u64(),
        Value::String(s) => s.parse::<u64>().ok(),
        _ => None,
    })
}

fn param_as_string(params: &HashMap<String, Value>, key: &str) -> Option<String> {
    params.get(key).and_then(|value| match value {
        Value::String(s) => Some(s.clone()),
        Value::Number(num) => Some(num.to_string()),
        _ => None,
    })
}
