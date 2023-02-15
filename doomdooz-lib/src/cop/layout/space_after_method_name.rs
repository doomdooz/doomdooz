use crate::cop;
use crate::source;
use crate::types;

// https://github.com/rubocop/rubocop/blob/master/lib/rubocop/cop/layout/space_after_method_name.rb
static MSG: &str = "Do not put a space between a method name and the opening parenthesis.";
static COP_NAME: &str = "Layout/SpaceAfterMethodName";

pub fn init() {
    cop::register(COP_NAME);
    cop::register_node_handler("def", COP_NAME, on_def);
}

pub fn on_def(node: &types::Node, file: &source::File) {
    if let types::Node::Def(node) = node {
        if let Some(args) = &node.args {
            let expr = args.expression();
            let loc = types::Loc {
                begin: expr.begin - 1,
                end: expr.begin,
            };

            if let Some(b) = file.as_bytes().get(loc.begin) {
                if *b == b' ' {
                    file.add_offense(COP_NAME, loc, MSG);
                    file.add_correction(types::Correction {
                        loc,
                        value: "".to_owned(),
                    });
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
        def foo (x) end
               ^ Do not put a space between a method name and the opening parenthesis.
        "};

        crate::expect_offense! {"
        def method= (y) end
                   ^ Do not put a space between a method name and the opening parenthesis.
        "};

        crate::expect_correction!(
            "def foo (x) end\ndef method= (y) end",
            "def foo(x) end\ndef method=(y) end"
        );
    }
}
