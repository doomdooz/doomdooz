pub mod naming;
pub mod style;

use crate::types;
use crate::NODE_HANDLERS;
use crate::OFFENSES;
use lib_ruby_parser::Node;
use std::sync::Mutex;

pub fn init() {
    naming::init();
    style::init();
}

pub fn register_node_handler(node_name: &'static str, handler: types::NodeHandler) {
    let mut map = NODE_HANDLERS.lock().unwrap();

    let entry = map.entry(node_name).or_insert(vec![]);
    entry.push(handler);
}
