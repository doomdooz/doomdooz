pub mod bundler;
pub mod naming;
pub mod style;

use crate::types;
use crate::COPS;
use crate::NODE_HANDLERS;
use crate::TOKENS_HANLDERS;
use std::sync::Once;

static INIT: Once = Once::new();

pub fn init() {
    INIT.call_once(|| {
        naming::init();
        style::init();
        bundler::init();
    });
}

pub fn register_node_handler(
    node_name: &'static str,
    cop_name: &'static str,
    handler: types::NodeHandler,
) {
    let mut map = NODE_HANDLERS.lock().unwrap();

    let entry = map.entry(node_name).or_insert(vec![]);
    entry.push((cop_name, handler));
}

pub fn register_tokens_handler(handler: types::TokensHandler, cop_name: &'static str) {
    TOKENS_HANLDERS.lock().unwrap().push((cop_name, handler));
}

pub fn register(cop_name: &'static str) {
    COPS.lock().unwrap().push(cop_name);
}
