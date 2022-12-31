use crate::cop;
use crate::cop::register_node_handler;
use crate::source;
use crate::types;
use std::str;

static COP_NAME: &str = "Style/AndOr";
const MSG: &str = "Use `&&` instead of `and`.";

pub fn init() {
    register_node_handler("and", COP_NAME, on_and);

    cop::register(COP_NAME);
}

pub fn on_and(node: &types::Node, file: &source::File) {
    if let types::Node::And(node) = node {
        let operator = file.source(node.operator_l);

        if operator == "and" {
            file.add_offense(COP_NAME, node.operator_l, MSG);
        }
    }
}

mod tests {
    #[test]
    fn test_and_operations() {
        crate::expect_offense! {"
            true and false
                 ^^^ Use `&&` instead of `and`.
        "};

        crate::expect_no_offense!("true && false");
    }
}
