use crate::source;
pub use lib_ruby_parser::{
    source::DecodedInput, Bytes, Lexer, Loc, Node, Parser, ParserOptions, ParserResult, Token,
};
use std::collections::HashMap;
use std::collections::HashSet;
use std::sync::Mutex;

pub type OffenseList = Mutex<Vec<String>>;
pub type NodeHandler = fn(&Node, &source::File);
pub type TokensHandler = fn(&Vec<Token>, &source::File);
pub type NodeHandlersMap = Mutex<HashMap<&'static str, Vec<(&'static str, NodeHandler)>>>;
pub type TargetFilesMap = HashMap<String, HashSet<&'static str>>;
