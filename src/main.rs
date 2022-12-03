use lib_ruby_parser::{Parser, ParserOptions};
use std::sync::Mutex;

#[macro_use]
extern crate lazy_static;
// use lazy_static;

mod cop;
mod reporting;

lazy_static! {
    static ref OFFENSES: Mutex<Vec<String>> = Mutex::new(vec![String::from("nothing")]);
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let options = ParserOptions {
        buffer_name: "(eval)".to_string(),
        ..Default::default()
    };
    let parser = Parser::new(String::from("      foo∂∂bar = baz"), options);

    let result = parser.do_parse();

    cop::naming::ascii_identifiers(result);

    reporting::print_report();

    Ok(())
}
