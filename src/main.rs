#![allow(dead_code, unused_imports)]
use lib_ruby_parser::Node;
use lib_ruby_parser::Token;
use std::collections::HashMap;
use std::env;
use std::sync::Mutex;

#[macro_use]
extern crate lazy_static;

mod cop;
mod source;
mod testing;
mod types;

lazy_static! {
    static ref NODE_HANDLERS: types::NodeHandlersMap = Mutex::new(HashMap::new());
    static ref TOKENS_HANLDERS: Mutex<Vec<types::TokensHandler>> = Mutex::new(vec![]);
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    cop::init();

    let args: Vec<String> = env::args().skip(1).collect();

    if args.len() != 1 {
        println!("pass a file");
        return Ok(());
    }

    for filepath in args {
        let file = source::File::new(filepath);
        file.process();
        file.print_report();
    }

    Ok(())
}
