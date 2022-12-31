use crate::cop;
use crate::cop::register_node_handler;
use crate::source;
use crate::types;

static MSG: &str = "Avoid the use of `END` blocks. Use `Kernel#at_exit` instead.";
static COP_NAME: &str = "Style/EndBlock";

pub fn init() {
    register_node_handler("postexe", COP_NAME, on_postexe);

    cop::register(COP_NAME);
}

pub fn on_postexe(node: &types::Node, file: &source::File) {
    if let types::Node::Postexe(node) = node {
        file.add_offense(COP_NAME, node.keyword_l, MSG);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        crate::expect_offense! {"
            END { test }
            ^^^
        "};
    }
}
