use std::collections::HashMap;
use std::sync::Mutex;

#[macro_use]
extern crate lazy_static;

mod config;
mod cop;
mod source;
mod target_finder;
mod testing;
mod types;

lazy_static! {
    static ref NODE_HANDLERS: types::NodeHandlersMap = Mutex::new(HashMap::new());
    static ref TOKENS_HANLDERS: Mutex<Vec<(&'static str, types::TokensHandler)>> =
        Mutex::new(vec![]);
    static ref COPS: Mutex<Vec<&'static str>> = Mutex::new(vec![]);
    static ref CONFIG: config::Config = config::load();
    static ref TARGET_FILES: types::TargetFilesMap = Mutex::new(HashMap::new());
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    cop::init();
    target_finder::scan();

    let mut inspected_files: usize = 0;
    let mut offenses: usize = 0;

    let files = TARGET_FILES.lock().unwrap();

    for (filepath, active_cops) in files.iter() {
        let file = source::File::new(filepath.clone(), active_cops);
        file.process();
        file.print_report();
        inspected_files += 1;
        offenses += file.total_offenses();
    }

    println!(
        "{} files inspected, {} offenses detected, XXX offenses autocorrectable",
        inspected_files, offenses
    );

    Ok(())
}
