#[macro_export]
macro_rules! expect_offense {
    ($source:expr) => {
        crate::cop::init();

        let mut active_cops: std::collections::HashSet<&str> = std::collections::HashSet::new();
        active_cops.insert(super::COP_NAME);

        let file = crate::source::File::inline($source, &active_cops);

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

        let file = crate::source::File::inline($source, &active_cops);
        file.process();

        assert_eq!(file.total_offenses(), 0);
    };
}
