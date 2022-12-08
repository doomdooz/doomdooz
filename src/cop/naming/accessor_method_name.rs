use crate::cop::register_node_handler;
use crate::source;
use crate::types;
use lib_ruby_parser::nodes;
use lib_ruby_parser::source::DecodedInput;
use lib_ruby_parser::Node;
use lib_ruby_parser::ParserResult;
use std::sync::Mutex;

const MSG_READER: &str = "Do not prefix reader method names with `get_`.";
const MSG_WRITER: &str = "Do not prefix writer method names with `set_`.";

static COP_NAME: &str = "Naming/AccessorMethodName";

pub fn init() {
    register_node_handler("def", on_def);
}

pub fn on_def(node: &Node, file: &source::File) {
    if let Node::Def(node) = node {
        if node.name.starts_with("get_") {
            if None == node.args {
                file.add_offense(COP_NAME, node.name_l.begin..node.name_l.end, MSG_READER);
            }
        } else if node.name.starts_with("set_") {
            if let Some(args) = &node.args {
                if let Node::Args(args) = &**args {
                    if args.args.len() == 1 {
                        file.add_offense(COP_NAME, node.name_l.begin..node.name_l.end, MSG_WRITER);
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_detects_get_attribute() {
        crate::expect_offense!(
            "
            def get_attribute
            end
        "
        );
    }

    #[test]
    fn it_detects_set_attribute() {
        crate::expect_offense!(
            "
            def set_attribute(aa)
            end
        "
        );
    }

    #[test]
    fn it_works_fine_with_other_method_names() {
        crate::expect_no_offense!(
            "
            def set_name
            end
        "
        );
    }
}
