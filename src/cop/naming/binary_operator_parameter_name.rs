use crate::cop::register_node_handler;
use crate::source;
use crate::types;
use lib_ruby_parser::nodes;
use lib_ruby_parser::source::DecodedInput;
use lib_ruby_parser::Node;
use lib_ruby_parser::ParserResult;
use std::sync::Mutex;

const MSG: &str = "When defining the %OPT% operator, name its argument `other`.";
const METHODS: [&str; 9] = ["+", "-", "[]", "[]=", "<<", "===", "=~", "eql?", "equal?"];

static COP_NAME: &str = "Naming/BinaryOperatorParameterName";

pub fn init() {
    register_node_handler("def", on_def);
}

pub fn on_def(node: &Node, file: &source::File) {
    if let Node::Def(node) = node {
        if METHODS.contains(&node.name.as_str()) {
            if let Some(args) = &node.args {
                if let Node::Args(args) = &**args {
                    let args = &args.args;
                    if args.len() == 1 {
                        if let Node::Arg(arg) = args.first().unwrap() {
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
