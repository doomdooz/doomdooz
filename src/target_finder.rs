use crate::COPS;
use std::{env, fs};

pub fn scan() {
    for entry in fs::read_dir(env::current_dir().unwrap()).unwrap() {
        let entry = &entry.unwrap();
        dbg!(entry);
        dbg!(&entry.path());
    }

    for cop in COPS.lock().unwrap().iter() {
        dbg!(cop.cop_name);
    }
}
