use crate::types;
use crate::CONFIG;
use crate::NODE_HANDLERS;
use crate::TOKENS_HANLDERS;
use std::fs;
use std::ops::Range;
use std::str;
use std::sync::Mutex;

pub struct File {
    filepath: String,
    offenses: types::OffenseList,
    pub parser_result: types::ParserResult,
}

impl<'a> File {
    #[cfg(test)]
    pub fn inline(source: &'static str) -> File {
        let options = types::ParserOptions {
            ..Default::default()
        };

        let parser = types::Parser::new(source, options);

        let parser_result = parser.do_parse();

        File {
            filepath: "".to_string(),
            parser_result: parser_result,
            offenses: Mutex::new(vec![]),
        }
    }

    pub fn new(filepath: String) -> File {
        let options = types::ParserOptions {
            ..Default::default()
        };

        let source = fs::read_to_string(&filepath).unwrap();

        let parser = types::Parser::new(source, options);

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

    fn iterate_nodes(&self, node: &types::Node) {
        let node_type = node.str_type();

        if let Some(handlers) = NODE_HANDLERS.lock().unwrap().get(node_type) {
            for (cop_name, handler) in handlers {
                if CONFIG.is_enabled(cop_name) {
                    handler(node, self);
                }
            }
        }

        match node {
            types::Node::Begin(n) => {
                for statement in &n.statements {
                    self.iterate_nodes(&statement);
                }
            }
            types::Node::Module(n) => {
                if let Some(body) = &n.body {
                    self.iterate_nodes(&body);
                }
            }
            types::Node::Class(n) => {
                if let Some(body) = &n.body {
                    self.iterate_nodes(&body);
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
