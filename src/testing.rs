#[macro_export]
macro_rules! expect_offense {
    ($source:expr) => {
        super::init();

        let mut active_cops: std::collections::HashSet<&str> = std::collections::HashSet::new();
        active_cops.insert(super::COP_NAME);

        let file = crate::source::File::inline($source, &active_cops);

        file.process();

        assert_eq!(file.total_offenses(), 1);

        crate::NODE_HANDLERS.lock().unwrap().clear();
        crate::TOKENS_HANLDERS.lock().unwrap().clear();
    };
}

#[macro_export]
macro_rules! expect_no_offense {
    ($source:expr) => {
        super::init();

        let mut active_cops: std::collections::HashSet<&str> = std::collections::HashSet::new();
        active_cops.insert(super::COP_NAME);

        let file = crate::source::File::inline($source, &active_cops);
        file.process();

        assert_eq!(file.total_offenses(), 0);

        crate::NODE_HANDLERS.lock().unwrap().clear();
        crate::TOKENS_HANLDERS.lock().unwrap().clear();
    };
}
