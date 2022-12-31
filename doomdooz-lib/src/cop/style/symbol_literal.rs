use crate::cop;
use crate::cop::register_node_handler;
use crate::source;
use crate::types;
use regex::Regex;
use std::str;

static MSG: &str = "Do not use strings for word-like symbol literals.";
static COP_NAME: &str = "Style/SymbolLiteral";

pub fn init() {
    register_node_handler("sym", COP_NAME, on_sym);

    cop::register(COP_NAME);
}

pub fn on_sym(node: &types::Node, file: &source::File) {
    let re = Regex::new(r#"\A:["'][A-Za-z_]\w*["']\z"#).unwrap();

    if let types::Node::Sym(node) = node {
        let literal = str::from_utf8(
            &file.parser_result.input.bytes[node.expression_l.begin..node.expression_l.end],
        )
        .unwrap();

        if re.is_match(literal) {
            file.add_offense(COP_NAME, node.expression_l, MSG);
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        crate::expect_offense2! {"
            :\"name\"
            ^^^^^^^
        "};

        crate::expect_no_offense!(":\"full name\"");
    }
}
