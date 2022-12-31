use crate::cop;
use crate::cop::register_tokens_handler;
use crate::source;
use crate::types;

static MSG: &str = "Do not leave space between `!` and its argument.";
static COP_NAME: &str = "Layout/SpaceAfterNot";

pub fn init() {
    register_tokens_handler(on_tokens, COP_NAME);

    cop::register(COP_NAME);
}

pub fn on_tokens(tokens: &Vec<types::Token>, file: &source::File) {
    let space = " ".as_bytes()[0];

    for token in tokens {
        dbg!(token.token_name());

        if token.token_name() == "tBANG" {
            if let Some(byte) = file.parser_result.input.bytes.get(token.loc.begin + 1) {
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
        crate::expect_offense! {"
            !   something
            ^ Do not leave space between `!` and its argument.
        "};

        crate::expect_no_offense! {"
            !something
        "};
    }
}
