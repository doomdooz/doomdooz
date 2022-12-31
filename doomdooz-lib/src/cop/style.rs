pub mod alias;
pub mod and_or;
pub mod array_join;
pub mod begin_block;
pub mod colon_method_definition;
pub mod empty_method;
pub mod end_block;
pub mod send;
pub mod strip;
pub mod symbol_literal;

pub fn init() {
    alias::init();
    and_or::init();
    array_join::init();
    begin_block::init();
    colon_method_definition::init();
    empty_method::init();
    end_block::init();
    send::init();
    strip::init();
    symbol_literal::init();
}
