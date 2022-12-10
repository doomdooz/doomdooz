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

// #[derive(Default)]
pub struct Config<'a> {
    pub cop_name: &'static str,
    pub enabled: bool,
    pub description: &'static str,
    pub style_guide: &'static str,
    pub include: Option<Vec<String>>,
    pub exclude: Option<Vec<String>>,
    pub parent_config: Option<&'a Config<'a>>,
    pub supported_styles: Option<Vec<String>>,
}
