// use lib_ruby_parser::parser::SymbolKind;
use crate::reporting::add_offense;
use lib_ruby_parser::Bytes;
use lib_ruby_parser::{ParserResult, Token};
use regex::Regex;
use std::ops::Range;

static IDENTIFIER_MSG: &str = "Use only ascii symbols in identifiers.";
static CONSTANT_MSG: &str = "Use only ascii symbols in constants.";
static COP_NAME: &str = "Naming/AsciiIdentifiers";

pub fn run(result: ParserResult) {
    for token in &result.tokens {
        if should_scheck(&token) && !is_ascci(&token.token_value) {
            let offense = first_offense_range(&token);
            add_offense(&crate::OFFENSES, offense, IDENTIFIER_MSG);
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

    Range {
        start: token.loc.begin + mat.start(),
        end: token.loc.begin + mat.end(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::reporting;
    use crate::testing;

    #[test]
    fn ascii_variable_identifier() {
        testing::execute("name= 'aa'", run);

        assert_eq!(reporting::total(&crate::OFFENSES), 0);
    }

    #[test]
    fn non_ascii_variable_identifier() {
        testing::execute("foo∂∂bar = 'aa'", run);

        // assert_eq!(reporting::total(), 1);
    }
}
