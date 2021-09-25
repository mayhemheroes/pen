mod error;
pub mod record_element_resolver;
pub mod type_canonicalizer;
pub mod type_collector;
pub mod type_comparability_checker;
pub mod type_difference_calculator;
pub mod type_equality_checker;
pub mod type_existence_validator;
pub mod type_id_calculator;
pub mod type_resolver;
pub mod type_subsumption_checker;
pub mod union_type_creator;
pub mod union_type_member_calculator;

pub use error::TypeError;