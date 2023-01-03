use crate::cop;
use crate::cop::register_node_handler;
use crate::source;
use crate::types;

static MSG: &str = "Float out of range.";
static COP_NAME: &str = "Lint/FloatOutOfRange";

pub fn init() {
    cop::register(COP_NAME);
    register_node_handler("float", COP_NAME, on_float);
}

pub fn on_float(node: &types::Node, file: &source::File) {
    if let types::Node::Float(node) = node {
        let (_, decimal) = node.value.split_once('.').unwrap();

        if decimal.is_empty() {
            return;
        }

        if let Some((_, second)) = decimal.split_once('e') {
            let second = second.parse::<i32>().unwrap();

            if second > 100 || second < -100 {
                file.add_offense(COP_NAME, node.expression_l, MSG);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        crate::expect_no_offense!("0.0");
        crate::expect_no_offense!("1.1e-100");
        crate::expect_no_offense!("55.7e89");

        crate::expect_offense! {"
            9.9999e999
            ^^^^^^^^^^ Float out of range.
        "};

        crate::expect_offense! {"
            1.0e-400
            ^^^^^^^^ Float out of range.
        "};
    }
}
