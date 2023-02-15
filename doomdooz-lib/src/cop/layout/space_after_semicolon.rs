use crate::cop;
use crate::source;
use crate::types;

static MSG: &str = "Space missing after semicolon.";
static COP_NAME: &str = "Layout/SpaceAfterSemicolon";

pub fn init() {
    cop::register(COP_NAME);
    cop::register_file_handler(on_file, COP_NAME);
}

pub fn on_file(file: &source::File) {
    for token in &file.parser_result.tokens {
        if token.token_name() == "tSEMI" {
            if let Some(b) = file.as_bytes().get(token.loc.begin + 1) {
                if *b != b' ' {
                    file.add_offense(COP_NAME, token.loc, MSG);
                    file.add_correction(types::Correction::replace(
                        types::loc(token.loc.begin + 1, token.loc.begin + 1),
                        " ",
                    ));
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn it_works() {
        expect_offense! {"
        x = 1;y = 2
             ^ Space missing after semicolon.
        "};

        expect_correction!("x = 1;y = 2", "x = 1; y = 2");
        expect_no_offense!("x = 1; y = 2");

        expect_no_offense!("b = 'a ;'");
        expect_no_offense!("x = 1;");
        expect_no_offense!("test { ; }");
    }
}
