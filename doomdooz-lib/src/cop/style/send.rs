use crate::cop;
use crate::source;
use crate::types;

static MSG: &str = "Object#__send__` or `Object#public_send` to `send`.";
static COP_NAME: &str = "Style/Send";

pub fn init() {
    cop::register(COP_NAME);
    cop::register_node_handler("send", COP_NAME, on_def);
}

pub fn on_def(node: &types::Node, file: &source::File) {
    if let types::Node::Send(node) = node {
        if node.method_name == "send" {
            file.add_offense(COP_NAME, node.selector_l.unwrap(), MSG)
        }
    }
}
