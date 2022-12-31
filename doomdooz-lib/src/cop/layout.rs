pub mod space_after_comma;
pub mod space_after_semicolon;
pub mod space_before_comma;
pub mod space_before_semicolon;

pub fn init() {
    space_after_comma::init();
    space_after_semicolon::init();
    space_before_comma::init();
    space_before_semicolon::init();
}
