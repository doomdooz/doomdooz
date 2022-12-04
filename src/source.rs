use crate::cop;
use lib_ruby_parser::{Parser, ParserOptions};
use std::fs;

pub struct File {
    filepath: String,
}

impl File {
    pub fn new(filepath: String) -> File {
        File { filepath }
    }

    pub fn parse(self) {
        let options = ParserOptions {
            ..Default::default()
        };

        let source = fs::read_to_string(self.filepath).unwrap();

        let parser = Parser::new(source, options);

        let result = parser.do_parse();

        cop::naming::ascii_identifiers(result);
    }
}
