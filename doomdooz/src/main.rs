use clap::{command, Arg, ArgAction};
use doomdooz_lib::{cop, source, target_finder, CONFIG, COPS};
use rayon::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    cop::init();

    let matches = command!() // requires `cargo` feature
        .arg(Arg::new("file").action(ArgAction::Append))
        .arg(
            Arg::new("list-target-files")
                .short('L')
                .long("list-target-files")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("autocorrect")
                .short('a')
                .long("autocorrect")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("show-cops")
                .long("show-cops")
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

    if matches.get_flag("show-cops") {
        show_cops();
        return Ok(());
    }

    if files.len() > 0 {
        panic!("passing files as argument is not supported yet");
    }

    run(matches.get_flag("autocorrect"));

    Ok(())
}

fn run(correction: bool) {
    // too slow
    let files = target_finder::scan();

    files
        .par_iter()
        .map(|(filepath, active_cops)| {
            let file = source::File::new(&filepath, active_cops);
            file.process();
            file.print_report();

            if correction {
                file.save_corrected();
            }
        })
        .count();
}

fn show_cops() {
    for cop in COPS.lock().unwrap().iter() {
        println!("{}\t\t{}", cop, CONFIG.get_string(cop, "Description"));
    }
}

fn print_target_files() {
    for (filepath, _) in target_finder::scan() {
        println!("{}", filepath.strip_prefix("./").unwrap());
    }
}
