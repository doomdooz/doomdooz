use std::env;
use std::sync::Mutex;

#[macro_use]
extern crate lazy_static;

mod cop;
mod reporting;
mod source;

lazy_static! {
    static ref OFFENSES: Mutex<Vec<String>> = Mutex::new(vec![]);
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.len() != 1 {
        println!("pass a file");
        return Ok(());
    }

    for filepath in args {
        source::File::new(filepath).parse();
    }

    reporting::print_report();

    Ok(())
}
