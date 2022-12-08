use crate::cop;
use crate::types;
use crate::NODE_HANDLERS;
use crate::TOKENS_HANLDERS;
use lib_ruby_parser::source::DecodedInput;
use lib_ruby_parser::Node;
use lib_ruby_parser::{Parser, ParserOptions, ParserResult};
use std::fs;
use std::ops::Range;
use std::str;
use std::sync::Mutex;

pub struct File {
    filepath: String,
    offenses: types::OffenseList,
    pub parser_result: ParserResult,
}

impl<'a> File {
    pub fn inline(source: &'static str) -> File {
        let options = ParserOptions {
            ..Default::default()
        };

        let parser = Parser::new(source, options);

        let parser_result = parser.do_parse();

        File {
            filepath: "".to_string(),
            parser_result: parser_result,
            offenses: Mutex::new(vec![]),
        }
    }

    pub fn new(filepath: String) -> File {
        let options = ParserOptions {
            ..Default::default()
        };

        let source = fs::read_to_string(&filepath).unwrap();

        let parser = Parser::new(source, options);

        let parser_result = parser.do_parse();

        File {
            filepath: filepath,
            parser_result: parser_result,
            offenses: Mutex::new(vec![]),
        }
    }

    pub fn process(&self) {
        let ast = self.parser_result.ast.as_ref();

        if let Some(ast) = ast {
            self.iterate_nodes(&*ast);

            for handler in TOKENS_HANLDERS.lock().unwrap().iter() {
                handler(&self.parser_result.tokens, self);
            }
        }
    }

    fn iterate_nodes(&self, node: &Node) {
        let node_type = node.str_type();

        if let Some(handlers) = NODE_HANDLERS.lock().unwrap().get(node_type) {
            for handler in handlers {
                handler(node, self);
            }
        }

        match node {
            Node::Begin(n) => {
                for statement in &n.statements {
                    self.iterate_nodes(&statement);
                }
            }
            _ => (),
        }
    }

    pub fn add_offense(&self, cop_name: &'static str, range: Range<usize>, message: &'static str) {
        let (line, col) = self
            .parser_result
            .input
            .line_col_for_pos(range.start)
            .unwrap();

        let source_line = &self.parser_result.input.lines[line];
        let line_string = &self.parser_result.input.bytes[source_line.start..source_line.end];

        let (_, col_end) = self
            .parser_result
            .input
            .line_col_for_pos(range.end)
            .unwrap();

        let annotation = format!("{}{}", " ".repeat(col), "^".repeat(col_end - col));

        let msg = format!(
            "{}:{}:{}: {} {}\n{}{}",
            self.filepath,
            line + 1,
            col + 1,
            cop_name,
            message.to_string(),
            str::from_utf8(line_string).unwrap(),
            annotation
        );

        self.offenses.lock().unwrap().push(msg);
    }

    pub fn print_report(&self) {
        self.offenses
            .lock()
            .unwrap()
            .iter()
            .for_each(|x| println!("{x}"));
    }

    pub fn total_offenses(&self) -> usize {
        self.offenses.lock().unwrap().len()
    }
}
