use crate::cop;
use crate::source;
use crate::types;
use regex::Regex;

static IDENTIFIER_MSG: &str = "Use only ascii symbols in identifiers.";
// static CONSTANT_MSG: &str = "Use only ascii symbols in constants.";
static COP_NAME: &str = "Naming/AsciiIdentifiers";

pub fn init() {
    cop::register(COP_NAME);
    cop::register_file_handler(on_file, COP_NAME);
}

pub fn on_file(file: &source::File) {
    for token in &file.parser_result.tokens {
        if should_scheck(&token) && !is_ascci(&token.token_value) {
            let offense = first_offense_range(&token);
            file.add_offense(COP_NAME, offense, IDENTIFIER_MSG);
        }
    }
}

fn is_ascci(bytes: &types::Bytes) -> bool {
    bytes.to_string().unwrap().is_ascii()
}

fn should_scheck(token: &types::Token) -> bool {
    token.token_name() == "tIDENTIFIER"
}

fn first_offense_range(token: &types::Token) -> types::Loc {
    let re = Regex::new(r"[^[:ascii:]]+").unwrap();
    let binding = token.token_value.to_string().unwrap();
    let mat = re.find(&binding).unwrap();

    types::Loc {
        begin: token.loc.begin + mat.start(),
        end: token.loc.begin + mat.end(),
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn ascii_variable_identifier() {
        crate::expect_no_offense!("name = 'aaa'");
    }

    #[test]
    fn non_ascii_variable_identifier() {
        crate::expect_offense! {"
            foo∂∂bar = 'aa'
               ^^^^^^ Use only ascii symbols in identifiers.
        "};
    }
}
