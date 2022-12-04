use crate::OFFENSES;
use std::ops::Range;

pub fn add_offense(range: Range<usize>, message: &'static str) {
    let msg = format!("{}:{} {}", range.start, range.end, message.to_string());
    OFFENSES.lock().unwrap().push(msg);
}

pub fn print_report() {
    OFFENSES
        .lock()
        .unwrap()
        .iter()
        .for_each(|x| println!("{x}"));
}
