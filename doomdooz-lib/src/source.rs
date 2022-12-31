use crate::types;
use crate::NODE_HANDLERS;
use crate::TOKENS_HANLDERS;
use std::cell::RefCell;
use std::collections::HashSet;
use std::fs;
use std::str;

pub struct File<'a> {
    filepath: String,
    offenses: types::OffenseList2,
    active_cops: &'a HashSet<&'a str>,
    pub parser_result: types::ParserResult,
}

impl<'a> File<'a> {
    pub fn inline(source: String, active_cops: &'a HashSet<&str>) -> File<'a> {
        let options = types::ParserOptions {
            ..Default::default()
        };

        let parser = types::Parser::new(source, options);

        let parser_result = parser.do_parse();

        File {
            filepath: "".to_string(),
            parser_result: parser_result,
            active_cops: active_cops,
            offenses: RefCell::new(vec![]),
        }
    }

    pub fn new(filepath: String, active_cops: &'a HashSet<&str>) -> File<'a> {
        let options = types::ParserOptions {
            ..Default::default()
        };

        let source = fs::read_to_string(&filepath).unwrap();

        let parser = types::Parser::new(source, options);

        let parser_result = parser.do_parse();

        File {
            filepath: filepath,
            parser_result: parser_result,
            active_cops: active_cops,
            offenses: RefCell::new(vec![]),
        }
    }

    pub fn is_enabled(&self, cop_name: &'static str) -> bool {
        self.active_cops.contains(cop_name)
    }

    pub fn process(&self) {
        let ast = self.parser_result.ast.as_ref();

        if let Some(ast) = ast {
            self.iterate_nodes(&*ast);
            for (cop_name, handler) in TOKENS_HANLDERS.lock().unwrap().iter() {
                if self.is_enabled(cop_name) {
                    handler(&self.parser_result.tokens, self);
                }
            }
        }
    }

    fn iterate_nodes(&self, node: &types::Node) {
        let call_handlers = |node_type| {
            if let Some(handlers) = NODE_HANDLERS.lock().unwrap().get(node_type) {
                for (cop_name, handler) in handlers {
                    if self.is_enabled(cop_name) {
                        handler(node, self);
                    }
                }
            }
        };

        call_handlers(node.str_type());

        // TODO: implement all types
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
            types::Node::Block(n) => {
                if let Some(body) = &n.body {
                    self.iterate_nodes(&body);
                }
            }
            types::Node::Lvasgn(n) => {
                if let Some(body) = &n.value {
                    self.iterate_nodes(&body);
                }
            }
            types::Node::Send(n) => {
                call_handlers(&("send_".to_owned() + &n.method_name));
            }
            _ => (),
        }
    }

    pub fn add_offense<T: AsRef<str> + std::fmt::Display>(
        &self,
        cop_name: &'static str,
        range: types::Loc,
        message: T,
    ) {
        let (line, col) = self
            .parser_result
            .input
            .line_col_for_pos(range.begin)
            .unwrap();

        let source_line = &self.parser_result.input.lines[line];
        let line_string = &self.parser_result.input.bytes[source_line.start..source_line.end];

        let (_, col_end) = self
            .parser_result
            .input
            .line_col_for_pos(range.end)
            .unwrap();

        let offense = types::Offense {
            filepath: self.filepath.clone(),
            line: line + 1,
            col_begin: col + 1,
            col_end: col_end + 1,
            message: message.to_string(),
            line_string: str::from_utf8(line_string).unwrap().to_string(),
            cop_name: cop_name.to_owned(),
        };

        self.offenses.borrow_mut().push(offense);
    }

    pub fn print_report(&self) {
        self.offenses
            .borrow()
            .iter()
            .for_each(|x| println!("{}", x.to_string()));
    }

    pub fn report(&self) -> String {
        let mut output = String::new();

        self.offenses
            .borrow()
            .iter()
            .for_each(|x| output.push_str(&x.to_string()));

        output
    }

    pub fn total_offenses(&self) -> usize {
        self.offenses.borrow().len()
    }
}
