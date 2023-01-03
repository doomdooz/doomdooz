use crate::cop;
use crate::cop::register_tokens_handler;
use crate::source;
use crate::types;

static MSG: &str = "Put a space before an end-of-line comment.";
static COP_NAME: &str = "Layout/SpaceBeforeComment";

pub fn init() {
    register_tokens_handler(on_tokens, COP_NAME);

    cop::register(COP_NAME);
}

pub fn on_tokens(_tokens: &Vec<types::Token>, file: &source::File) {
    for comment in &file.parser_result.comments {
        if comment.location.begin == 0 {
            continue;
        }

        let chr_loc = types::Loc {
            begin: comment.location.begin - 1,
            end: comment.location.begin,
        };

        let chr = file.source(chr_loc);

        if chr != " " {
            let loc = types::Loc {
                begin: chr_loc.begin + 1,
                end: chr_loc.end + 1,
            };
            file.add_offense(COP_NAME, loc, MSG);

            file.add_correction(types::Correction {
                loc: loc,
                value: " #".to_owned(),
            });
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        crate::expect_offense! {"
            a += 1# increment
                  ^ Put a space before an end-of-line comment.
        "};

        crate::expect_no_offense!("a += 1 # increment");
        crate::expect_no_offense!("# comment");
        crate::expect_no_offense! {"
          =begin
          Doc comment
          =end
        "};
        crate::expect_offense! {"
          <<~STR# my string
                ^ Put a space before an end-of-line comment.
            text
          STR
        "};
    }

    #[test]
    fn it_registers_an_offense_and_corrects_after_a_heredoc() {
        crate::expect_correction!(
            {
                "
          <<~STR# my string
            text
          STR
        "
            },
            {
                "
          <<~STR # my string
            text
          STR
        "
            }
        );
    }
}
