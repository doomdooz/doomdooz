use std::collections::HashMap;
use std::sync::Mutex;

#[macro_use]
extern crate lazy_static;

#[cfg(test)]
#[macro_use]
extern crate indoc;

pub mod config;
pub mod cop;
pub mod source;
pub mod target_finder;
pub mod testing;
pub mod types;

lazy_static! {
    static ref NODE_HANDLERS: types::NodeHandlersMap = Mutex::new(HashMap::new());
    static ref FILE_HANLDERS: Mutex<Vec<(&'static str, types::FileHandler)>> = Mutex::new(vec![]);
    pub static ref COPS: Mutex<Vec<&'static str>> = Mutex::new(vec![]);
    pub static ref CONFIG: config::Config = config::load();
}
