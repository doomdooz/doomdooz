use crate::cop;
use crate::types;

static COP_NAME: &str = "Bundler/DuplicatedGem";

pub fn init() {
    cop::register(COP_NAME);
}
