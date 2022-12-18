use doomdooz_lib::{cop, source, target_finder};
use rayon::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    cop::init();

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
