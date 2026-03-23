pub mod utils;

use crate::registry::{Registry, CommandEntry, CommandSource};

pub fn register(registry: &mut Registry) {
    registry.register(CommandEntry {
        site: "bilibili".to_string(),
        name: "feed".to_string(),
        description: "Get bilibili feed with WBI signature".to_string(),
        source: CommandSource::Native("bilibili_feed".to_string()),
    });
}
