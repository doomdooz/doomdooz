use doomdooz_lib::{cop, source, COPS};
use std::collections::HashSet;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn doomdooz(source: String) -> String {
    cop::init();

    let cops = HashSet::from_iter(COPS.lock().unwrap().iter().cloned());
    let file = source::File::inline(source, &cops);

    file.process();
    file.report()
}
