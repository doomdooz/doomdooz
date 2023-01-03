pub mod bundler;
pub mod layout;
pub mod lint;
pub mod naming;
pub mod style;

use crate::types;
use crate::COPS;
use crate::FILE_HANLDERS;
use crate::NODE_HANDLERS;
use std::sync::Once;

static INIT: Once = Once::new();

pub fn init() {
    INIT.call_once(|| {
        bundler::init();
        layout::init();
        lint::init();
        naming::init();
        style::init();
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

pub fn register_file_handler(handler: types::FileHandler, cop_name: &'static str) {
    FILE_HANLDERS.lock().unwrap().push((cop_name, handler));
}

pub fn register(cop_name: &'static str) {
    COPS.lock().unwrap().push(cop_name);
}
