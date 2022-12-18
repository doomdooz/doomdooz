use crate::cop;
use crate::cop::register_tokens_handler;
use crate::source;
use crate::types;

static MSG: &str = "Space found before semicolon.";
static COP_NAME: &str = "Layout/SpaceBeforeSemicolon";

pub fn init() {
    register_tokens_handler(on_tokens, COP_NAME);

    cop::register(COP_NAME);
}

pub fn on_tokens(_tokens: &Vec<types::Token>, file: &source::File) {
    let space = " ".as_bytes()[0];
    let semicolon = ";".as_bytes()[0];

    let mut space_seen = false;

    for (location, byte) in file.parser_result.input.bytes.iter().enumerate() {
        if *byte == semicolon && space_seen {
            file.add_offense(
                COP_NAME,
                types::Loc {
                    begin: location,
                    end: location + 1,
                },
                MSG,
            );
        } else {
            space_seen = *byte == space;
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        crate::expect_offense!("x = 1 ; y = 2");

        crate::expect_no_offense!("x = 1; y = 2");
    }
}
