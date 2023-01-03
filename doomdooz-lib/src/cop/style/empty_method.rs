use crate::cop;
use crate::source;
use crate::types;

static MSG: &str = "Put empty method definitions on a single line.";
static COP_NAME: &str = "Style/EmptyMethod";

pub fn init() {
    cop::register(COP_NAME);
    cop::register_node_handler("def", COP_NAME, on_def);
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
                file.add_offense(COP_NAME, node.name_l, MSG);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        crate::expect_offense! {"
            def name
                ^^^^ Put empty method definitions on a single line.
            end
        "};
    }
}
