use crate::source;
pub use lib_ruby_parser::{
    source::DecodedInput, Bytes, Lexer, Loc, Node, Parser, ParserOptions, ParserResult, Token,
};
use std::cell::RefCell;
use std::collections::HashMap;
use std::collections::HashSet;
use std::sync::Mutex;

pub type OffenseList = Mutex<Vec<String>>;
pub type OffenseList2 = RefCell<Vec<Offense>>;
pub type NodeHandler = fn(&Node, &source::File);
pub type TokensHandler = fn(&Vec<Token>, &source::File);
pub type NodeHandlersMap = Mutex<HashMap<&'static str, Vec<(&'static str, NodeHandler)>>>;
pub type TargetFilesMap = HashMap<String, HashSet<&'static str>>;

#[derive(Debug)]
pub struct Correction {
    pub loc: Loc,
    pub value: String,
}

pub struct Offense {
    pub filepath: String,
    pub line: usize,
    pub col_begin: usize,
    pub col_end: usize,
    pub message: String,
    pub line_string: String,
    pub cop_name: String,
}

impl Offense {
    pub fn to_string(&self) -> String {
        let annotation = format!(
            "{}{}",
            " ".repeat(self.col_begin - 1),
            "^".repeat(self.col_end - self.col_begin)
        );

        format!(
            "{}:{}:{}: {} {}\n{}{}\n",
            self.filepath,
            self.line,
            self.col_begin,
            self.cop_name,
            self.message,
            self.line_string,
            annotation
        )
    }

    pub fn test_report(&self) -> String {
        let annotation = format!(
            "{}{} {}",
            " ".repeat(self.col_begin - 1),
            "^".repeat(self.col_end - self.col_begin),
            self.message
        );

        format!("{}\n{}", self.line_string.trim_end(), annotation)
    }
}

#[cfg(test)]
mod tests {
    use super::Offense;

    #[test]
    fn offense_to_string() {
        let offense = Offense {
            filepath: "test.rb".to_owned(),
            line: 10,
            col_begin: 3,
            col_end: 15,
            message: "Something".to_owned(),
            line_string: "  def something\n".to_owned(),
            cop_name: "Style/Test".to_owned(),
        };

        let output = indoc! {"
            test.rb:10:3: Style/Test Something
              def something
              ^^^^^^^^^^^^
            "};

        assert_eq!(offense.to_string(), output);
    }
}
