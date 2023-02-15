use crate::cop;
use crate::source;

static COP_NAME: &str = "Layout/SpaceBeforeComma";

pub fn init() {
    cop::register(COP_NAME);
    cop::register_file_handler(on_file, COP_NAME);
}

pub fn on_file(file: &source::File) {
    cop::space_before_punctuation(COP_NAME, file, "tCOMMA", "comma");
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn it_works() {
        expect_offense! {"
        each { |s , t| }
                 ^ Space found before comma.
        "};
        expect_offense! {"
        each { |s   , t| }
                 ^^^ Space found before comma.
        "};

        expect_correction! {"
        each { |s , t| }
        ", "
        each { |s, t| }
        "};

        // registers an offense and corrects array index with space before comma
        expect_offense! {"
        formats[0 , 1]
                 ^ Space found before comma.
        "};
        expect_correction! {"formats[0 , 1]", "formats[0, 1]"};

        // registers an offense and corrects method call arg with space before comma
        expect_offense! {"
        a(1 , 2)
           ^ Space found before comma.
        "}
        expect_correction! {"a(1 , 2)", "a(1, 2)"};

        // registers an offense and corrects method call arg with space before comma
        expect_offense! {"
        each { |s  , t| a(1  , formats[0  , 1])}
                 ^^ Space found before comma.
                           ^^ Space found before comma.
                                        ^^ Space found before comma.
        "};
    }
}
