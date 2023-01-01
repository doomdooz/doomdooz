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
        .arg(
            Arg::new("list-target-files")
                .short('L')
                .long("list-target-files")
                .action(ArgAction::SetTrue),
        )
        .get_matches();

    let files = matches
        .get_many::<String>("file")
        .unwrap_or_default()
        .map(|v| v.as_str())
        .collect::<Vec<_>>();

    if matches.get_flag("list-target-files") {
        print_target_files();
        return Ok(());
    }

    if files.len() > 0 {
        panic!("passing files as argument is not supported yet");
    }

    run();

    Ok(())
}

fn run() {
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
}

fn print_target_files() {
    for (filepath, _) in target_finder::scan() {
        println!("{}", filepath.strip_prefix("./").unwrap());
    }
}
