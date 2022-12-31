use crate::cop;
use crate::cop::register_node_handler;
use crate::source;
use crate::types;

static MSG: &str = "Do not use `::` for defining class methods.";
static COP_NAME: &str = "Style/ColonMethodDefinition";

pub fn init() {
    register_node_handler("defs", COP_NAME, on_defs);

    cop::register(COP_NAME);
}

pub fn on_defs(node: &types::Node, file: &source::File) {
    if let types::Node::Defs(node) = node {
        if file.source(node.operator_l) == "::" {
            file.add_offense(COP_NAME, node.operator_l, MSG)
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        crate::expect_no_offense! {"
          class Foo
            def self.bar
              something
            end
          end
        "};

        crate::expect_offense! {"
            class Foo
              def self::bar
                      ^^ Do not use `::` for defining class methods.
                something
              end
            end
        "};
    }
}
