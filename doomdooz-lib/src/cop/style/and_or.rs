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
        let operator = str::from_utf8(
            &file.parser_result.input.bytes[node.operator_l.begin..node.operator_l.end],
        );

        if let Ok(operator) = operator {
            if operator == "and" {
                file.add_offense(COP_NAME, node.operator_l.begin..node.operator_l.end, MSG);
            }
        }
    }
}
