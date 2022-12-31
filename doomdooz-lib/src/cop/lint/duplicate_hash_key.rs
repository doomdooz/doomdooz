use crate::cop;
use crate::cop::register_node_handler;
use crate::source;
use crate::types;
use std::collections::HashSet;

static MSG: &str = "Duplicated key in hash literal.";
static COP_NAME: &str = "Lint/DuplicateHashKey";

pub fn init() {
    register_node_handler("hash", COP_NAME, on_hash);

    cop::register(COP_NAME);
}

pub fn on_hash(node: &types::Node, file: &source::File) {
    let mut keys: HashSet<String> = HashSet::new();

    if let types::Node::Hash(node) = node {
        for pair in &node.pairs {
            if let types::Node::Pair(pair) = pair {
                let key: String = match &*pair.key {
                    types::Node::Str(n) => n.value.to_string().unwrap(),
                    types::Node::Sym(n) => n.name.to_string().unwrap(),
                    _ => "".to_string(),
                };

                if !key.is_empty() {
                    if keys.contains(&key) {
                        file.add_offense(COP_NAME, pair.expression_l, MSG);
                    } else {
                        keys.insert(key);
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_detects_no_offense() {
        crate::expect_no_offense!("hash = { name: 'mohsen', age: '33' }");
    }

    #[test]
    fn it_detects_offense() {
        crate::expect_offense2! {"
            hash = { name: 'mohsen', name: 'other' }
                                     ^^^^^^^^^^^^^
        "};
    }
}
