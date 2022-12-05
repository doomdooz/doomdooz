use crate::reporting;
use lib_ruby_parser::nodes;
use lib_ruby_parser::Node;
use lib_ruby_parser::ParserResult;

const MSG_READER: &str = "Do not prefix reader method names with `get_`.";
const MSG_WRITER: &str = "Do not prefix writer method names with `set_`.";

static COP_NAME: &str = "Naming/AccessorMethodName";

pub fn run(result: ParserResult) {
    // dbg!(&result.ast);
    find_defs(*result.ast.unwrap());
}

fn find_defs(node: Node) {
    match node {
        Node::Def(n) => on_def(n),
        Node::Begin(n) => {
            for statement in n.statements {
                find_defs(statement);
            }
        }
        _ => println!("not found"),
    }
}

pub fn on_def(node: nodes::Def) {
    if node.name.starts_with("get_") {
        if None == node.args {
            reporting::add_offense(node.name_l.begin..node.name_l.end, MSG_READER);
            // dbg!("found wrong getter");
        }
    }

    dbg!(node);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::reporting;
    use crate::testing;

    #[test]
    fn it_works() {
        let code = "
            def get_attribute
            end
        ";

        testing::execute(code, run);

        // assert_eq!(reporting::total(), 1);
    }
}
