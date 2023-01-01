use doomdooz_lib::{cop, source, target_finder, COPS};
use rayon::prelude::*;

use clap::{command, Arg, ArgAction};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    cop::init();

    {
        println!(
            "there are {} cops implemented so far",
            COPS.lock().unwrap().len()
        );
    }

    let matches = command!() // requires `cargo` feature
        .arg(Arg::new("file").action(ArgAction::Append))
        .get_matches();

    let files = matches
        .get_many::<String>("file")
        .unwrap_or_default()
        .map(|v| v.as_str())
        .collect::<Vec<_>>();

    if files.len() > 0 {
        panic!("passing files as argument is not supported yet");
    }

    // too slow
    let files = target_finder::scan();

    files
        .par_iter()
        .map(|(filepath, active_cops)| {
            let file = source::File::new(filepath.clone(), active_cops);
            file.process();
            file.print_report();
        })
        .count();

    Ok(())
}
