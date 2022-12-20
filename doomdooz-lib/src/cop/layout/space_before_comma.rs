use crate::cop;
use crate::cop::register_tokens_handler;
use crate::source;
use crate::types;

static MSG: &str = "Space found before comma.";
static COP_NAME: &str = "Layout/SpaceBeforeComma";

pub fn init() {
    register_tokens_handler(on_tokens, COP_NAME);

    cop::register(COP_NAME);
}

pub fn on_tokens(tokens: &Vec<types::Token>, file: &source::File) {
    let space = " ".as_bytes()[0];

    for token in tokens {
        if token.token_name() == "tCOMMA" {
            if let Some(byte) = file.parser_result.input.bytes.get(token.loc.begin - 1) {
                if *byte == space {
                    file.add_offense(COP_NAME, token.loc, MSG);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        crate::expect_offense!("[1 , 2]");

        crate::expect_no_offense!("[1, 2]");
    }
}
