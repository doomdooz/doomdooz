pub mod bundler;
pub mod layout;
pub mod lint;
pub mod naming;
pub mod style;

use crate::source;
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

pub fn space_before_punctuation(
    cop_name: &'static str,
    file: &source::File,
    token_name: &str,
    kind: &str,
) {
    for token in file.parser_result.tokens.iter() {
        if token.token_name() != token_name {
            continue;
        }

        let loc = types::loc(token.loc.begin - 1, token.loc.end - 1);

        let mut sp_n = 0;
        let bytes = file.as_bytes();
        while let Some(b) = bytes.get(loc.begin - sp_n) {
            if *b == b' ' {
                sp_n += 1;
            } else {
                break;
            }
        }
        if sp_n == 0 {
            continue;
        }

        let loc = types::loc(loc.begin - (sp_n - 1), loc.end);
        file.add_offense(cop_name, loc, format!("Space found before {kind}."));
        file.add_correction(types::Correction::remove(loc));
    }
}
