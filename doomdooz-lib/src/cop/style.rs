pub mod alias;
pub mod and_or;
pub mod empty_method;

pub fn init() {
    alias::init();
    and_or::init();
    empty_method::init();
}
