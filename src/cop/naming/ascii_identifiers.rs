use crate::cop::register_tokens_handler;
use crate::source;
use lib_ruby_parser::Bytes;
use lib_ruby_parser::{ParserResult, Token};
use regex::Regex;
use std::ops::Range;

static IDENTIFIER_MSG: &str = "Use only ascii symbols in identifiers.";
static CONSTANT_MSG: &str = "Use only ascii symbols in constants.";
static COP_NAME: &str = "Naming/AsciiIdentifiers";

pub fn init() {
    register_tokens_handler(on_tokens);
}

pub fn on_tokens(tokens: &Vec<Token>, file: &source::File) {
    for token in tokens {
        if should_scheck(&token) && !is_ascci(&token.token_value) {
            let offense = first_offense_range(&token);
            file.add_offense(COP_NAME, offense, IDENTIFIER_MSG);
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
    use crate::testing::*;

    #[test]
    fn ascii_variable_identifier() {
        super::init();

        expect_no_offense("name = 'aaa'");
    }

    #[test]
    fn non_ascii_variable_identifier() {
        expect_offense("foo∂∂bar = 'aa'");
    }
}
