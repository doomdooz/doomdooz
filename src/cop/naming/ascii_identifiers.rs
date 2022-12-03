// use lib_ruby_parser::parser::SymbolKind;
use crate::reporting::add_offense;
use lib_ruby_parser::Bytes;
use lib_ruby_parser::{ParserResult, Token};
use regex::Regex;
use std::ops::Range;

static IDENTIFIER_MSG: &str = "Use only ascii symbols in identifiers.";
static CONSTANT_MSG: &str = "Use only ascii symbols in constants.";

pub fn ascii_identifiers(result: ParserResult) {
    for token in result.tokens {
        if should_scheck(&token) && !is_ascci(&token.token_value) {
            println!("{:#?}", token);
            let offense = first_offense_range(&token);
            add_offense(offense, IDENTIFIER_MSG);
        }
    }
}

fn is_ascci(bytes: &Bytes) -> bool {
    bytes.to_string().unwrap().is_ascii()
}

fn should_scheck(token: &Token) -> bool {
    token.token_name() == "tIDENTIFIER"
}

fn first_offense_range(token: &Token) -> Range<usize> {
    let re = Regex::new(r"[^[:ascii:]]+").unwrap();
    let binding = token.token_value.to_string().unwrap();
    let mat = re.find(&binding).unwrap();

    dbg!(Range {
        start: token.loc.begin + mat.start(),
        end: token.loc.begin + mat.end(),
    })
}

// fn add_offense(range: Range<usize>, message: &'static str) {
//     dbg!(range);
//     dbg!(message);
// }
