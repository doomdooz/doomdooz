use lib_ruby_parser::{Parser, ParserOptions};

mod cop;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let options = ParserOptions {
        buffer_name: "(eval)".to_string(),
        ..Default::default()
    };
    let parser = Parser::new(String::from("      foo∂∂bar = baz"), options);

    let result = parser.do_parse();

    cop::naming::ascii_identifiers(result);

    Ok(())
}
