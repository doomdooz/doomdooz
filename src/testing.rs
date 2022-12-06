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
