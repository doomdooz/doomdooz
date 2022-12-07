use crate::cop::register_node_handler;
use crate::source;
use crate::types;
use lib_ruby_parser::nodes;
use lib_ruby_parser::source::DecodedInput;
use lib_ruby_parser::Node;
use lib_ruby_parser::ParserResult;
use std::sync::Mutex;

static COP_NAME: &str = "Style/EmptyMethod";
static MSG: &str = "Put empty method definitions on a single line.";

pub fn init() {
    register_node_handler("def", on_def);
}

pub fn on_def(node: &Node, file: &source::File) {
    if let Node::Def(node) = node {
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

        // if node.name.starts_with("get_") {
        //     if None == node.args {
        //         file.add_offense(COP_NAME, node.name_l.begin..node.name_l.end, MSG_READER);
        //     }
        // } else if node.name.starts_with("set_") {
        //     if let Some(args) = &node.args {
        //         if let Node::Args(args) = &**args {
        //             if args.args.len() == 1 {
        //                 file.add_offense(COP_NAME, node.name_l.begin..node.name_l.end, MSG_WRITER);
        //             }
        //         }
        //     }
        // }
    }
}
