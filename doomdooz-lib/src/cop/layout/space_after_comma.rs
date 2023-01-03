use crate::cop;
use crate::source;

static MSG: &str = "Space missing after comma.";
static COP_NAME: &str = "Layout/SpaceAfterComma";

pub fn init() {
    cop::register(COP_NAME);
    cop::register_file_handler(on_file, COP_NAME);
}

pub fn on_file(file: &source::File) {
    let valid_list = [b' ', b'\n', b')', b'}'];

    for token in &file.parser_result.tokens {
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
