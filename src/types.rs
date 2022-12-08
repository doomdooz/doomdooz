use crate::source;
use lib_ruby_parser::source::DecodedInput;
use lib_ruby_parser::Node;
use lib_ruby_parser::Token;
use std::collections::HashMap;
use std::sync::Mutex;

pub type OffenseList = Mutex<Vec<String>>;
pub type NodeHandler = fn(&Node, &source::File);
pub type TokensHandler = fn(&Vec<Token>, &source::File);
pub type NodeHandlersMap = Mutex<HashMap<&'static str, Vec<NodeHandler>>>;
