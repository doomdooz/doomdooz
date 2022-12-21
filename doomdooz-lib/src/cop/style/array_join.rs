use crate::cop;
use crate::cop::register_node_handler;
use crate::source;
use crate::types;

static MSG: &str = "Favor `Array#join` over `Array#*`.";
static COP_NAME: &str = "Style/ArrayJoin";

pub fn init() {
    register_node_handler("send", COP_NAME, on_def);

    cop::register(COP_NAME);
}

pub fn on_def(node: &types::Node, file: &source::File) {
    if let types::Node::Send(node) = node {
        if node.method_name == "*" {
            if let Some(recv) = &node.recv {
                if let types::Node::Array(_recv) = &**recv {
                    if node.args.len() == 1 {
                        let arg = node.args.get(0).unwrap();
                        if let types::Node::Str(_arg) = arg {
                            file.add_offense(COP_NAME, node.selector_l.unwrap(), MSG)
                        }
                    }
                }
            }
        }
    }
}
