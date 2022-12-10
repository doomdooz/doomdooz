pub mod default;
pub mod naming;
pub mod style;

use crate::types;
use crate::COPS;
use crate::NODE_HANDLERS;
use crate::TOKENS_HANLDERS;

#[cfg(not(test))]
pub fn init() {
    naming::init();
    style::init();
}

#[cfg(test)]
pub fn init() {}

pub fn register_node_handler(node_name: &'static str, handler: types::NodeHandler) {
    let mut map = NODE_HANDLERS.lock().unwrap();

    let entry = map.entry(node_name).or_insert(vec![]);
    entry.push(handler);
}

pub fn register_tokens_handler(handler: types::TokensHandler) {
    TOKENS_HANLDERS.lock().unwrap().push(handler);
}

pub fn register(cop: &'static types::Cop) {
    COPS.lock().unwrap().push(cop);
}
