pub mod alias;
pub mod and_or;
pub mod begin_block;
pub mod empty_method;
pub mod end_block;

pub fn init() {
    alias::init();
    and_or::init();
    begin_block::init();
    empty_method::init();
    end_block::init();
}
