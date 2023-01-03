use crate::cop;
use crate::source;
use crate::types;

static MSG: &str = "Avoid the use of `BEGIN` blocks.";
static COP_NAME: &str = "Style/BeginBlock";

pub fn init() {
    cop::register(COP_NAME);
    cop::register_node_handler("preexe", COP_NAME, on_preexe);
}

pub fn on_preexe(node: &types::Node, file: &source::File) {
    if let types::Node::Preexe(node) = node {
        file.add_offense(COP_NAME, node.keyword_l, MSG);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        crate::expect_offense! {"
            BEGIN { test }
            ^^^^^ Avoid the use of `BEGIN` blocks.
        "};
    }
}
