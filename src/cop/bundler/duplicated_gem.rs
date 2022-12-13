use crate::cop;
use crate::source;
use crate::types;
use std::collections::HashMap;
use std::sync::Mutex;

static COP_NAME: &str = "Bundler/DuplicatedGem";
static MSG: &str =
    "Gem `%gem_name%` requirements already given on line %line_number% of the Gemfile.";

lazy_static! {
    static ref GEMS: Mutex<HashMap<String, usize>> = Mutex::new(HashMap::new());
}

pub fn init() {
    cop::register(COP_NAME);
    cop::register_node_handler("send", COP_NAME, on_send);
}

pub fn on_send(node: &types::Node, file: &source::File) {
    let mut gems = GEMS.lock().unwrap();

    if let types::Node::Send(node) = node {
        if node.method_name == "gem" {
            if node.args.len() >= 1 {
                if let types::Node::Str(gem) = &node.args[0] {
                    let gem_name = gem.value.to_string().unwrap();

                    if let Some(line) = gems.get(&gem_name) {
                        let line = line + 1;

                        file.add_offense(
                            COP_NAME,
                            gem.expression_l.begin..gem.expression_l.end,
                            MSG.replace("%gem_name%", &gem_name)
                                .replace("%line_number%", &line.to_string()),
                        );
                    } else {
                        let (line, _) = file
                            .parser_result
                            .input
                            .line_col_for_pos(gem.expression_l.begin)
                            .unwrap();

                        gems.insert(gem_name, line); // add line
                    }
                }
            }
        }
    }
}
