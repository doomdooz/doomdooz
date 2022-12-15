use doomdooz_lib::{cop, source, target_finder, TARGET_FILES};
use rayon::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    cop::init();

    // too slow
    target_finder::scan();

    let files = TARGET_FILES.lock().unwrap();

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