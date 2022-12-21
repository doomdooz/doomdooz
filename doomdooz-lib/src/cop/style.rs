pub mod alias;
pub mod and_or;
pub mod array_join;
pub mod begin_block;
pub mod empty_method;
pub mod end_block;
pub mod send;
pub mod symbol_literal;

pub fn init() {
    alias::init();
    and_or::init();
    array_join::init();
    begin_block::init();
    empty_method::init();
    end_block::init();
    send::init();
    symbol_literal::init();
}
