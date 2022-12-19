pub mod space_before_comma;
pub mod space_before_semicolon;

pub fn init() {
    space_before_comma::init();
    space_before_semicolon::init();
}
