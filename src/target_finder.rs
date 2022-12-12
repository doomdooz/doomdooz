use crate::CONFIG;
use crate::COPS;
use crate::TARGET_FILES;
use globwalk;
use std::collections::HashSet;

pub fn scan() {
    // TODO: there is a lot of space to optimize this function
    let cops = COPS.lock().unwrap();
    let mut target_files = TARGET_FILES.lock().unwrap();

    for cop in cops.iter() {
        if CONFIG.is_enabled(cop) {
            let mut patterns = CONFIG.get_array(cop, "Include");

            for exclude in CONFIG.get_array(cop, "Exclude") {
                let string = String::from("!") + &exclude;
                patterns.push(string);
            }

            let walker = globwalk::GlobWalkerBuilder::from_patterns(".", &patterns)
                .file_type(globwalk::FileType::FILE)
                .build()
                .unwrap()
                .into_iter()
                .filter_map(Result::ok);

            for file in walker {
                let entry = &mut target_files
                    .entry(file.path().display().to_string())
                    .or_insert(HashSet::new());
                entry.insert(cop);
            }
        }
    }
}
