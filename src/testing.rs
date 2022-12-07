use lib_ruby_parser::{Node, Parser, ParserOptions, ParserResult};

pub fn parse(source: &str) -> ParserResult {
    let options = ParserOptions {
        ..Default::default()
    };

    let parser = Parser::new(source, options);

    parser.do_parse()
}

pub fn ast(source: &str) -> Node {
    let options = ParserOptions {
        ..Default::default()
    };

    let parser = Parser::new(source, options);

    *parser.do_parse().ast.unwrap()
}

pub fn execute(source: &str, cop_func: fn(ParserResult)) {
    let options = ParserOptions {
        ..Default::default()
    };

    let parser = Parser::new(source, options);

    let result = parser.do_parse();

    cop_func(result);
}

pub fn expect_no_offense(source: &'static str) {
    let file = crate::source::File::inline(source);
    file.process();

    assert_eq!(file.total_offenses(), 0);
}

pub fn expect_offense(source: &'static str) {
    let file = crate::source::File::inline(source);
    file.process();

    assert_eq!(file.total_offenses(), 1);
}
