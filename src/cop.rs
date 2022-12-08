pub mod naming;
pub mod style;

use crate::types;
use crate::NODE_HANDLERS;
use crate::TOKENS_HANLDERS;
use lib_ruby_parser::Node;
use lib_ruby_parser::Token;
use std::sync::Mutex;

#[cfg(test)]
pub fn init() {
    naming::init();
    style::init();
}

#[cfg(not(test))]
pub fn init() {}

pub fn register_node_handler(node_name: &'static str, handler: types::NodeHandler) {
    let mut map = NODE_HANDLERS.lock().unwrap();

    let entry = map.entry(node_name).or_insert(vec![]);
    entry.push(handler);
}

pub fn register_tokens_handler(handler: types::TokensHandler) {
    TOKENS_HANLDERS.lock().unwrap().push(handler);
}
