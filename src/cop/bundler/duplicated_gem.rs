use crate::cop;

static COP_NAME: &str = "Bundler/DuplicatedGem";

pub fn init() {
    cop::register(COP_NAME);
}
