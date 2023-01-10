use crate::cop;
use crate::cop::register_node_handler;
use crate::source;
use crate::types;

static MSG: &str =
    "Do not use regexp literal as a condition. The regexp literal matches `$_` implicitly.";
static COP_NAME: &str = "Lint/RegexpAsCondition";

pub fn init() {
    cop::register(COP_NAME);
    register_node_handler("match_current_line", COP_NAME, on_match_current_line);
}

pub fn on_match_current_line(node: &types::Node, file: &source::File) {
    if let types::Node::MatchCurrentLine(node) = node {
        file.add_offense(COP_NAME, node.expression_l, MSG);

        file.add_correction(types::Correction {
            loc: node.expression_l,
            value: file.source(node.expression_l) + " =~ $_",
        });
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        crate::expect_offense! {"
          if /foo/
             ^^^^^ Do not use regexp literal as a condition. The regexp literal matches `$_` implicitly.
          end
        "};

        crate::expect_offense! {"
            if !/foo/
                ^^^^^ Do not use regexp literal as a condition. The regexp literal matches `$_` implicitly.
            end
        "};

        crate::expect_no_offense!("/foo/");
        // crate::expect_no_offense!("!/foo/");

        crate::expect_no_offense! {"
            if /foo/ =~ str
            end
        "};

        crate::expect_correction!(
            {
                "
            if /foo/
            end
        "
            },
            {
                "
            if /foo/ =~ $_
            end
        "
            }
        );

        crate::expect_correction!(
            {
                "
            if !/foo/
            end
        "
            },
            {
                "
            if !/foo/ =~ $_
            end
        "
            }
        );
    }
}
