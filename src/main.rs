// #![allow(dead_code, unused_imports)]
use glob::glob;
use std::collections::HashMap;
use std::env;
use std::sync::Mutex;

#[macro_use]
extern crate lazy_static;

mod cop;
mod source;
mod target_finder;
mod testing;
mod types;

lazy_static! {
    static ref NODE_HANDLERS: types::NodeHandlersMap = Mutex::new(HashMap::new());
    static ref TOKENS_HANLDERS: Mutex<Vec<types::TokensHandler>> = Mutex::new(vec![]);
    static ref COPS: Mutex<Vec<&'static types::Cop<'static>>> = Mutex::new(vec![]);
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    cop::init();

    target_finder::scan();

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
