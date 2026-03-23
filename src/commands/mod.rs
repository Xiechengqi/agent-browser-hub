pub mod bilibili;

use crate::registry::Registry;

pub fn register_all(registry: &mut Registry) {
    bilibili::register(registry);
}
