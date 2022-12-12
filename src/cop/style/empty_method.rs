use crate::cop;
use crate::cop::register_node_handler;
use crate::source;
use crate::types;

static MSG: &str = "Put empty method definitions on a single line.";
static COP_NAME: &str = "Style/EmptyMethod";

pub fn init() {
    register_node_handler("def", COP_NAME, on_def);

    cop::register(COP_NAME);
}

pub fn on_def(node: &types::Node, file: &source::File) {
    if let types::Node::Def(node) = node {
        if let None = node.body {
            let (name_line, _) = file
                .parser_result
                .input
                .line_col_for_pos(node.name_l.begin)
                .unwrap();
            let (end_line, _) = file
                .parser_result
                .input
                .line_col_for_pos(node.end_l.unwrap().begin)
                .unwrap();

            if name_line != end_line {
                file.add_offense(COP_NAME, node.keyword_l.begin..node.name_l.end, MSG);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        crate::expect_offense!(
            "
            def name
            end
        "
        );
    }
}
