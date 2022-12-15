pub mod accessor_method_name;
pub mod ascii_identifiers;
pub mod binary_operator_parameter_name;

pub fn init() {
    accessor_method_name::init();
    ascii_identifiers::init();
    binary_operator_parameter_name::init();
}
