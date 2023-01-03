use crate::cop;
use crate::source;
use crate::types;
use crate::CONFIG;

static COP_NAME: &str = "Style/Alias";

const MSG_ALIAS: &str = "Use `alias_method` instead of `alias`.";
const MSG_ALIAS_METHOD: &str = "Use `alias` instead of `alias_method` %<current>s.";

pub fn init() {
    cop::register(COP_NAME);
    cop::register_node_handler("alias", COP_NAME, on_alias);
    cop::register_node_handler("send_alias_method", COP_NAME, on_alias_method);
}

pub fn on_alias(node: &types::Node, file: &source::File) {
    if CONFIG.get_string(COP_NAME, "EnforcedStyle") == "prefer_alias_method" {
        if let types::Node::Alias(node) = node {
            file.add_offense(COP_NAME, node.expression_l, MSG_ALIAS_METHOD);
        }
    }
}

pub fn on_alias_method(node: &types::Node, file: &source::File) {
    if CONFIG.get_string(COP_NAME, "EnforcedStyle") == "prefer_alias" {
        if let types::Node::Send(node) = node {
            file.add_offense(COP_NAME, node.expression_l, MSG_ALIAS);
        }
    }
}
