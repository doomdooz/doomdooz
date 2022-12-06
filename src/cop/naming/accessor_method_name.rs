use crate::cop::register_node_handler;
use crate::reporting;
use crate::types;
use lib_ruby_parser::nodes;
use lib_ruby_parser::Node;
use lib_ruby_parser::ParserResult;
use std::sync::Mutex;

const MSG_READER: &str = "Do not prefix reader method names with `get_`.";
const MSG_WRITER: &str = "Do not prefix writer method names with `set_`.";

static COP_NAME: &str = "Naming/AccessorMethodName";

pub fn init() {
    register_node_handler("def", on_def);
}

pub fn run(result: ParserResult) {
    // find_defs(*result.ast.unwrap());
}

// fn find_defs(node: Node) {
//     match node {
//         Node::Def(n) => on_def(n),
//         Node::Begin(n) => {
//             for statement in n.statements {
//                 find_defs(statement);
//             }
//         }
//         _ => println!("not found"),
//     }
// }

pub fn on_def(node: &Node, offenses: types::OffenseList) {
    if let Node::Def(node) = node {
        if node.name.starts_with("get_") {
            if None == node.args {
                reporting::add_offense(offenses, node.name_l.begin..node.name_l.end, MSG_READER);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::reporting;
    use crate::testing;
    use crate::types;
    use std::sync::Mutex;

    #[test]
    fn it_detects_get_attribute() {
        let code = "
            def get_attribute
            end
        ";

        let offenses: types::OffenseList = &Mutex::new(vec![]);

        on_def(&testing::ast(code), &offenses);

        assert_eq!(reporting::total(offenses), 1);
    }
}
