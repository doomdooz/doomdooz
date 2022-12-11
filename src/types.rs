use crate::source;
pub use lib_ruby_parser::{
    source::DecodedInput, Bytes, Node, Parser, ParserOptions, ParserResult, Token,
};
use std::collections::HashMap;
use std::sync::Mutex;

pub type OffenseList = Mutex<Vec<String>>;
pub type NodeHandler = fn(&Node, &source::File);
pub type TokensHandler = fn(&Vec<Token>, &source::File);
pub type NodeHandlersMap = Mutex<HashMap<&'static str, Vec<NodeHandler>>>;

pub struct Cop<'a> {
    pub cop_name: &'static str,
    pub enabled: bool,
    pub description: &'static str,
    pub style_guide: &'static str,
    pub include: Option<Vec<String>>,
    pub exclude: Option<Vec<String>>,
    pub parent_config: Option<&'a Cop<'a>>,
    pub supported_styles: Option<Vec<String>>,
}

impl Cop<'_> {
    pub fn find_targets(self) {
        let include_patterns: &Vec<String>;

        if let Some(_) = self.include {
            include_patterns = &self.include.unwrap();
        } else {
            include_patterns = &(&self.parent_config).unwrap().include.as_ref().unwrap();
        }
    }
}
