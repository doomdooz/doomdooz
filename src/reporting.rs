use crate::OFFENSES;
use std::ops::Range;

pub fn add_offense(range: Range<usize>, message: &'static str) {
    OFFENSES.lock().unwrap().push(message.to_string());
}

pub fn print_report() {
    OFFENSES
        .lock()
        .unwrap()
        .iter()
        .for_each(|x| println!("{x}"));
}
