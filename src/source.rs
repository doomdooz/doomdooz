use crate::cop;
use crate::NODE_HANDLERS;
use crate::OFFENSES;
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

        let result = parser.do_parse();

        iterate_nodes(&(*result.ast.unwrap()));
    }
}

fn iterate_nodes(node: &Node) {
    // run_node(node.str_type)
    let node_type = node.str_type();

    if let Some(handlers) = NODE_HANDLERS.lock().unwrap().get(node_type) {
        for handler in handlers {
            // dbg!(crate::OFFENSES);
            handler(node, &*OFFENSES);
        }
    }

    dbg!(node.str_type());

    // self
    // match node {
    //     Node::Def(n) => self.run_node(n),
    //     Node::Begin(n) => {
    //         for statement in n.statements {
    //             iterate_nodes(statement);
    //         }
    //     }
    //     _ => println!("not found"),
    // }
}

// fn run_node(self, node_name: &'static str) {
//     // for (node_name, handler) in NODE_HANDLERS.lock().unwrap().iter() {
//     //     dbg!(node_name);
//     // }
// }
