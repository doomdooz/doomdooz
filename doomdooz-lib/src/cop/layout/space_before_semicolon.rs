use crate::cop;
use crate::source;

static COP_NAME: &str = "Layout/SpaceBeforeSemicolon";

pub fn init() {
    cop::register(COP_NAME);
    cop::register_file_handler(on_file, COP_NAME);
}

pub fn on_file(file: &source::File) {
    cop::space_before_punctuation(COP_NAME, file, "tSEMI", "semicolon");
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn it_works() {
        expect_offense! {"
        x = 1 ; y = 2
             ^ Space found before semicolon.
        "};
        expect_offense! {"
        x = 1   ; y = 2
             ^^^ Space found before semicolon.
        "};
        expect_correction! {"x = 1 ; y = 2", "x = 1; y = 2"};

        expect_no_offense!("x = 1; y = 2");
        expect_no_offense!("b = 'a ;'");
    }
}
