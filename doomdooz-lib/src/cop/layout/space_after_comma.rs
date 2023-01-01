use crate::cop;
use crate::cop::register_tokens_handler;
use crate::source;
use crate::types;

static MSG: &str = "Space missing after comma.";
static COP_NAME: &str = "Layout/SpaceAfterComma";

pub fn init() {
    register_tokens_handler(on_tokens, COP_NAME);

    cop::register(COP_NAME);
}

pub fn on_tokens(tokens: &Vec<types::Token>, file: &source::File) {
    let valid_list = [b' ', b'\n', b')', b'}'];

    for token in tokens {
        if token.token_name() == "tCOMMA" {
            if let Some(byte) = file.parser_result.input.bytes.get(token.loc.begin + 1) {
                if !valid_list.contains(byte) {
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
        crate::expect_offense! {"
            [1,2]
              ^ Space missing after comma.
        "};

        crate::expect_no_offense!("[1, 2]");
    }
}
