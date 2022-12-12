use crate::CONFIG;
use crate::COPS;
use crate::TARGET_FILES;
use glob::glob;
use std::collections::HashSet;
use std::{env, fs};

pub fn scan() {
    // TODO: there is a lot of space to optimize this function
    let cops = COPS.lock().unwrap();
    let mut target_files = TARGET_FILES.lock().unwrap();

    for cop in cops.iter() {
        if CONFIG.is_enabled(cop) {
            let mut include_list: Vec<String> = vec![];
            let mut exclude_list: Vec<String> = vec![];
            let include_patterns = CONFIG.get_array(cop, "Include");
            let exclude_patterns = CONFIG.get_array(cop, "Exclude");
            dbg!(cop);
            dbg!(&include_patterns);
            dbg!(&exclude_patterns);

            for include_pattern in include_patterns {
                for entry in glob(include_pattern.as_ref()).unwrap() {
                    if let Ok(path) = entry {
                        include_list.push(path.display().to_string());
                    }
                }
            }

            for exclude_pattern in exclude_patterns {
                for entry in glob(exclude_pattern.as_ref()).unwrap() {
                    if let Ok(path) = entry {
                        exclude_list.push(path.display().to_string());
                    }
                }
            }

            for file in &include_list {
                if !exclude_list.contains(file) {
                    let entry = &mut target_files.entry(file.clone()).or_insert(HashSet::new());
                    entry.insert(cop);
                }
            }
        }
    }
}
