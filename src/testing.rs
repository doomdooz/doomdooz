use lib_ruby_parser::{Parser, ParserOptions, ParserResult};

pub fn execute(source: String, cop_func: fn(ParserResult)) {
    let options = ParserOptions {
        ..Default::default()
    };

    let parser = Parser::new(source, options);

    let result = parser.do_parse();

    cop_func(result);
}
