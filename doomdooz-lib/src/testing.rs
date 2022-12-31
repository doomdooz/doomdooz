#[macro_export]
macro_rules! expect_offense2 {
    ($source:expr) => {
        crate::cop::init();

        let mut active_cops: std::collections::HashSet<&str> = std::collections::HashSet::new();
        active_cops.insert(super::COP_NAME);

        let source = indoc! {$source};

        let mut source_lines: Vec<String> = vec![];
        let mut annotation_lines: Vec<String> = vec![];

        source.to_string().lines().for_each(|line| {
            if line.trim().starts_with("^") {
                annotation_lines.push(source_lines.last().unwrap().to_string());
                annotation_lines.push(line.to_string());
            } else {
                source_lines.push(line.to_string());
            }
        });

        let file = crate::source::File::inline(source_lines.join("\n"), &active_cops);

        file.process();

        assert_eq!(file.test_report(), annotation_lines.join("\n"));
    };
}

#[macro_export]
macro_rules! expect_offense {
    ($source:expr) => {
        crate::cop::init();

        let mut active_cops: std::collections::HashSet<&str> = std::collections::HashSet::new();
        active_cops.insert(super::COP_NAME);

        let file = crate::source::File::inline($source.to_string(), &active_cops);

        file.process();

        assert_eq!(file.total_offenses(), 1);
    };
}

#[macro_export]
macro_rules! expect_no_offense {
    ($source:expr) => {
        crate::cop::init();

        let mut active_cops: std::collections::HashSet<&str> = std::collections::HashSet::new();
        active_cops.insert(super::COP_NAME);

        let file = crate::source::File::inline($source.to_string(), &active_cops);
        file.process();

        assert_eq!(file.total_offenses(), 0);
    };
}
