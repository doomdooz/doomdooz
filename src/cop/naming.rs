pub mod accessor_method_name;
pub mod binary_operator_parameter_name;
// pub mod ascii_identifiers;

pub fn init() {
    accessor_method_name::init();
    binary_operator_parameter_name::init();
}
