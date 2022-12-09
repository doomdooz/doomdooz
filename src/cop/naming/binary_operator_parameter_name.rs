use crate::cop::register_node_handler;
use crate::source;
use crate::types;

const MSG: &str = "When defining the %OPT% operator, name its argument `other`.";
const METHODS: [&str; 9] = ["+", "-", "[]", "[]=", "<<", "===", "=~", "eql?", "equal?"];

static COP_NAME: &str = "Naming/BinaryOperatorParameterName";

pub fn init() {
    register_node_handler("def", on_def);
}

pub fn on_def(node: &types::Node, file: &source::File) {
    if let types::Node::Def(node) = node {
        if METHODS.contains(&node.name.as_str()) {
            if let Some(args) = &node.args {
                if let types::Node::Args(args) = &**args {
                    let args = &args.args;
                    if args.len() == 1 {
                        if let types::Node::Arg(arg) = args.first().unwrap() {
                            if arg.name != "other" {
                                file.add_offense(
                                    COP_NAME,
                                    arg.expression_l.begin..arg.expression_l.end,
                                    MSG,
                                );
                            }
                        }
                    }
                }
            }
        }
    }
}
