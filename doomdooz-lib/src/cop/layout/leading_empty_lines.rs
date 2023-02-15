use regex::Regex;

use crate::cop;
use crate::source;
use crate::types;

// https://github.com/rubocop/rubocop/blob/master/lib/rubocop/cop/layout/leading_empty_lines.rb
static MSG: &str = "Unnecessary blank line at the beginning of the source.";
static COP_NAME: &str = "Layout/LeadingEmptyLines";

lazy_static! {
    static ref RE_COMMENT: Regex = Regex::new(r"\A([#]+)[^#\s=+-]").unwrap();
}

pub fn init() {
    cop::register(COP_NAME);
    cop::register_file_handler(on_file, COP_NAME);
}

pub fn on_file(file: &source::File) {
    let token = &file.parser_result.tokens.get(0);
    if token.is_none() {
        return;
    }
    let token = token.unwrap();
    let (line, _) = file.line_col(token.loc.begin).unwrap();
    if line <= 1 {
        return;
    }

    file.add_offense(COP_NAME, token.loc, MSG);
    file.add_correction(types::Correction {
        loc: types::Loc {
            begin: 0,
            end: token.loc.begin,
        },
        value: "".to_string(),
    });
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn it_works() {
        expect_no_offense!("");
        expect_no_offense!("\n");

        expect_no_offense! {"
        class Foo
        end
        "}

        expect_no_offense! {"
        puts 1
        "}

        expect_no_offense! {"
        # something
        "}
    }

    #[test]
    fn test_class() {
        expect_offense! {"
        
        class Foo
        ^^^^^ Unnecessary blank line at the beginning of the source.
        end
        "}

        expect_correction! {"
        
        class Foo
        end", "
        class Foo
        end"}
    }

    #[test]
    fn test_line() {
        expect_offense! {"

        puts 1
        ^^^^ Unnecessary blank line at the beginning of the source.
        end
        "}

        expect_correction! {"
        
        puts 1", "puts 1"}
    }

    #[test]
    #[ignore = "Comment in lib-ruby-parser not is a token"]
    fn test_comment() {
        expect_offense! {"

        # something
        ^^^^^^^^^^^ Unnecessary blank line at the beginning of the source.
        test
        "}

        expect_correction! {"

        # something
        test", "
        # something
        test"};
    }

    #[test]
    fn test_class_with_multiple_empty_lines() {
        expect_offense! {"
        
        
        class Foo
        ^^^^^ Unnecessary blank line at the beginning of the source.
        end
        "}

        expect_correction! {"
        
        
        class Foo
        end", "
        class Foo
        end"}
    }
}
