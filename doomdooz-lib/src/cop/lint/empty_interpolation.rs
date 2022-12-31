use crate::cop;
use crate::cop::register_node_handler;
use crate::source;
use crate::types;

static MSG: &str = "Empty interpolation detected.";
static COP_NAME: &str = "Lint/EmptyInterpolation";

pub fn init() {
    register_node_handler("dstr", COP_NAME, on_dstr);

    cop::register(COP_NAME);
}

pub fn on_dstr(node: &types::Node, file: &source::File) {
    if let types::Node::Dstr(node) = node {
        for part in &node.parts {
            if let types::Node::Begin(part) = part {
                if part.statements.is_empty() {
                    file.add_offense(COP_NAME, part.expression_l, MSG);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        crate::expect_no_offense!(" \"something #{a}");
        crate::expect_offense! {"
            \"something #{}
                       ^^^
        "};
    }
}
