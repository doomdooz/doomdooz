use crate::cop;
use crate::source;

static MSG: &str = "Space found before semicolon.";
static COP_NAME: &str = "Layout/SpaceBeforeSemicolon";

pub fn init() {
    cop::register(COP_NAME);
    cop::register_file_handler(on_file, COP_NAME);
}

pub fn on_file(file: &source::File) {
    let space = " ".as_bytes()[0];

    for token in &file.parser_result.tokens {
        if token.token_name() == "tSEMI" {
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
        crate::expect_offense! {"
            x = 1 ; y = 2
                  ^ Space found before semicolon.
        "};

        crate::expect_no_offense!("x = 1; y = 2");

        crate::expect_no_offense!("b = 'a ;'");
    }
}
