use crate::cop;
use crate::cop::register_node_handler;
use crate::source;
use crate::types;

const MSG_READER: &str = "Do not prefix reader method names with `get_`.";
const MSG_WRITER: &str = "Do not prefix writer method names with `set_`.";

static COP_NAME: &str = "Naming/AccessorMethodName";

pub fn init() {
    register_node_handler("def", COP_NAME, on_def);
    cop::register(COP_NAME);
}

pub fn on_def(node: &types::Node, file: &source::File) {
    if let types::Node::Def(node) = node {
        if node.name.starts_with("get_") {
            if None == node.args {
                file.add_offense(COP_NAME, node.name_l, MSG_READER);
            }
        } else if node.name.starts_with("set_") {
            if let Some(args) = &node.args {
                if let types::Node::Args(args) = &**args {
                    if args.args.len() == 1 {
                        file.add_offense(COP_NAME, node.name_l, MSG_WRITER);
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
        crate::expect_offense! {"
            def get_attribute
                ^^^^^^^^^^^^^
            end
        "};
    }

    #[test]
    fn it_detects_set_attribute() {
        crate::expect_offense! {"
            def set_attribute(aa)
                ^^^^^^^^^^^^^
            end
        "};
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
