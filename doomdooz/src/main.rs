use std::time::Instant;

use doomdooz_lib::{cop, source, target_finder, CONFIG};
use rayon::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    cop::init();

    let now = Instant::now();

    // too slow
    let files = target_finder::scan();

    let diff = now.elapsed();

    println!("{:?}", diff);

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
