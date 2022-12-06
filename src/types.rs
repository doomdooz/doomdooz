use lib_ruby_parser::source::DecodedInput;
use lib_ruby_parser::Node;
use std::collections::HashMap;
use std::sync::Mutex;

pub type OffenseList<'a> = &'a Mutex<Vec<String>>;
pub type NodeHandler = fn(&Node, OffenseList, &DecodedInput);
pub type NodeHandlersMap = Mutex<HashMap<&'static str, Vec<NodeHandler>>>;
