use crate::cop;
use crate::source;
use crate::types;

// https://github.com/rubocop/rubocop/blob/master/lib/rubocop/cop/layout/space_after_colon.rb
static MSG: &str = "Space missing after colon.";
static COP_NAME: &str = "Layout/SpaceAfterColon";

pub fn init() {
    cop::register(COP_NAME);
    cop::register_node_handler("pair", COP_NAME, on_pair);
    cop::register_node_handler("kwoptarg", COP_NAME, on_kwoptarg);
}

pub fn on_kwoptarg(node: &types::Node, file: &source::File) {
    if let types::Node::Kwoptarg(pair) = node {
        let loc = pair.name_l;

        // Check next byte after colon, e.g.: `a:1`, b is `1`
        if let Some(b) = file.as_bytes().get(loc.end + 1) {
            if *b == b' ' {
                return;
            }

            let loc = types::Loc {
                begin: loc.end,
                end: loc.end + 1,
            };

            file.add_offense(COP_NAME, loc, MSG);
            file.add_correction(types::Correction::replace(loc, ": "));
        }
    }
}

pub fn on_pair(node: &types::Node, file: &source::File) {
    if let types::Node::Pair(pair) = node {
        let colon = file.source(&pair.operator_l);
        if colon != ":" {
            return;
        }

        let loc = pair.operator_l;

        // Check next byte after colon
        if let Some(b) = file.as_bytes().get(loc.end) {
            if *b == b' ' {
                return;
            }

            file.add_offense(COP_NAME, loc, MSG);
            file.add_correction(types::Correction {
                loc,
                value: ": ".to_owned(),
            });
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        crate::expect_offense! {"
        info = {a1:1,b: 2}
                  ^ Space missing after colon.
        "};

        crate::expect_offense! {"
        def f(a:, foo:2); end
                     ^ Space missing after colon.
        "};

        crate::expect_correction!("def f(a:, b:2); {a:3} end", "def f(a:, b: 2); {a: 3} end");

        crate::expect_no_offense!("def f(a:, b: 2); {a: 3}; end");
    }
}
