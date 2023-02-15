use crate::cop;
use crate::source;

static MSG: &str = "Do not leave space between `!` and its argument.";
static COP_NAME: &str = "Layout/SpaceAfterNot";

pub fn init() {
    cop::register(COP_NAME);
    cop::register_file_handler(on_file, COP_NAME);
}

pub fn on_file(file: &source::File) {
    for token in &file.parser_result.tokens {
        if token.token_name() == "tBANG" {
            if let Some(byte) = file.parser_result.input.bytes.get(token.loc.begin + 1) {
                if *byte == b' ' {
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
