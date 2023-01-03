use crate::cop;
use crate::source;
use crate::types;
use regex::Regex;
use std::str;

static MSG: &str = "Do not use strings for word-like symbol literals.";
static COP_NAME: &str = "Style/SymbolLiteral";

pub fn init() {
    cop::register(COP_NAME);
    cop::register_node_handler("sym", COP_NAME, on_sym);
}

pub fn on_sym(node: &types::Node, file: &source::File) {
    let re = Regex::new(r#"\A:["'][A-Za-z_]\w*["']\z"#).unwrap();

    if let types::Node::Sym(node) = node {
        let literal = &file.source(node.expression_l);

        if re.is_match(literal) {
            file.add_offense(COP_NAME, node.expression_l, MSG);
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        crate::expect_offense! {"
            :\"name\"
            ^^^^^^^ Do not use strings for word-like symbol literals.
        "};

        crate::expect_no_offense!(":\"full name\"");
    }
}
