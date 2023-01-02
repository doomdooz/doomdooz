use crate::cop;
use crate::cop::register_node_handler;
use crate::source;
use crate::types;
use std::str;

static COP_NAME: &str = "Style/AndOr";
const MSG_AND: &str = "Use `&&` instead of `and`.";
const MSG_OR: &str = "Use `||` instead of `or`.";

pub fn init() {
    register_node_handler("and", COP_NAME, on_and);
    register_node_handler("or", COP_NAME, on_or);

    cop::register(COP_NAME);
}

pub fn on_and(node: &types::Node, file: &source::File) {
    if let types::Node::And(node) = node {
        let operator = file.source(node.operator_l);

        if operator == "and" {
            file.add_offense(COP_NAME, node.operator_l, MSG_AND);

            file.add_correction(types::Correction {
                loc: node.operator_l,
                value: "&&".to_string(),
            });
        }
    }
}

pub fn on_or(node: &types::Node, file: &source::File) {
    if let types::Node::Or(node) = node {
        let operator = file.source(node.operator_l);

        if operator == "or" {
            file.add_offense(COP_NAME, node.operator_l, MSG_OR);

            file.add_correction(types::Correction {
                loc: node.operator_l,
                value: "||".to_string(),
            });
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

        crate::expect_offense! {"
            true or false
                 ^^ Use `||` instead of `or`.
        "};

        crate::expect_no_offense!("true && false");
        crate::expect_no_offense!("true || false");

        crate::expect_correction!("true and false", "true && false");
        crate::expect_correction!("true or false", "true || false");
    }
}
