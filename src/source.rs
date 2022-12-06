use crate::cop;
use crate::NODE_HANDLERS;
use crate::OFFENSES;
use lib_ruby_parser::source::DecodedInput;
use lib_ruby_parser::Node;
use lib_ruby_parser::{Parser, ParserOptions, ParserResult};
use std::fs;

pub struct File {
    filepath: String,
}

impl File {
    pub fn new(filepath: String) -> File {
        File { filepath }
    }

    pub fn parse(&mut self) {
        let options = ParserOptions {
            ..Default::default()
        };

        let source = fs::read_to_string(&self.filepath).unwrap();

        let parser = Parser::new(source, options);

        let ParserResult { ast, input, .. } = parser.do_parse();

        iterate_nodes(&(*ast.unwrap()), &input);
    }
}

fn iterate_nodes(node: &Node, input: &DecodedInput) {
    let node_type = node.str_type();

    if let Some(handlers) = NODE_HANDLERS.lock().unwrap().get(node_type) {
        for handler in handlers {
            handler(node, &*OFFENSES, input);
        }
    }

    match node {
        Node::Begin(n) => {
            for statement in &n.statements {
                iterate_nodes(&statement, input);
            }
        }
        _ => (),
    }
}
