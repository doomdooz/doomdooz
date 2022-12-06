use crate::types;
use crate::OFFENSES;
use std::ops::Range;

pub fn add_offense(offenses: types::OffenseList, range: Range<usize>, message: &'static str) {
    let msg = format!("{}:{} {}", range.start, range.end, message.to_string());
    offenses.lock().unwrap().push(msg);
}

pub fn total(offenses: types::OffenseList) -> usize {
    offenses.lock().unwrap().len()
}

pub fn print_report() {
    OFFENSES
        .lock()
        .unwrap()
        .iter()
        .for_each(|x| println!("{x}"));
}
