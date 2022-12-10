use crate::cop;
use crate::cop::register_node_handler;
use crate::source;
use crate::types;

static MSG: &str = "Put empty method definitions on a single line.";

lazy_static! {
    static ref COP: types::Cop<'static> = types::Cop {
        cop_name: "Style/EmptyMethod",
        enabled: true,
        description: "Checks the formatting of empty method definitions.",
        style_guide: "#no-single-line-methods",
        supported_styles: Some(vec!["compact".into(), "expanded".into()]),
        include: None,
        exclude: None,
        parent_config: None,
    };
}

pub fn init() {
    register_node_handler("def", on_def);

    cop::register(&COP);
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
                file.add_offense(COP.cop_name, node.keyword_l.begin..node.name_l.end, MSG);
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
