use crate::cop;
use crate::cop::register_node_handler;
use crate::source;
use crate::types;

static MSG: &str = "Use `strip` instead of `%<methods>s`.";
static COP_NAME: &str = "Style/Strip";

pub fn init() {
    register_node_handler("send", COP_NAME, on_lstrip_rstrip);

    cop::register(COP_NAME);
}

pub fn on_lstrip_rstrip(node: &types::Node, file: &source::File) {
    if let types::Node::Send(node) = node {
        if node.method_name == "lstrip" || node.method_name == "rstrip" {
            if let Some(recv) = &node.recv {
                if let types::Node::Send(recv) = &**recv {
                    if recv.method_name == "lstrip" || recv.method_name == "rstrip" {
                        let mut loc = node.selector_l.unwrap();
                        loc.begin = recv.selector_l.unwrap().begin;

                        // dbg!(&node);
                        file.add_offense(COP_NAME, loc, MSG);
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn strip_works() {
        crate::expect_offense! {"
            'str'.lstrip.rstrip
                  ^^^^^^^^^^^^^
            " };
    }
}
