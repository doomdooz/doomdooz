#![allow(dead_code, unused_imports)]
use glob::glob;
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

    let mut args: Vec<String> = env::args().skip(1).collect();
    let mut inspected_files: usize = 0;
    let mut offenses: usize = 0;

    if args.len() == 0 {
        args.push(String::from("**/*.rb"));
    }

    for filepath in args {
        for entry in glob(&filepath).unwrap() {
            if let Ok(path) = entry {
                let file = source::File::new(path.to_str().unwrap().to_string());
                file.process();
                file.print_report();
                inspected_files += 1;
                offenses += file.total_offenses();
            } else {
                panic!("error while reading path");
            }
        }
    }

    println!(
        "{} files inspected, {} offenses detected, XXX offenses autocorrectable",
        inspected_files, offenses
    );

    Ok(())
}
