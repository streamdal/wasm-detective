use crate::error::CustomError;
use ajson::Value;

pub mod detective;
pub mod error;
pub mod matcher_core;
pub mod matcher_numeric;
pub mod matcher_pii;

#[cfg(test)]
#[path = "matcher_numeric_tests.rs"]
mod matcher_numeric_tests;

#[cfg(test)]
#[path = "matcher_core_tests.rs"]
mod matcher_core_tests;

#[cfg(test)]
#[path = "matcher_pii_tests.rs"]
mod matcher_pii_tests;

#[cfg(test)]
#[path = "test_utils.rs"]
mod test_utils;

// For parse_field()
pub trait FromValue
where
    Self: Sized,
{
    fn from_value(value: &Value) -> Result<Self, CustomError>;
}

impl FromValue for f64 {
    fn from_value(value: &Value) -> Result<Self, CustomError> {
        value
            .as_f64()
            .ok_or(CustomError::Error("not a number".to_string()))
    }
}

// Not able to figure out the lifetime stuff here; going with a separate helper func just for Value for now
// impl FromValue for Value {
//     fn from_value(value: &Value) -> Result<Self, CustomError> {
//         Ok(value.clone())
//     }
// }

impl FromValue for String {
    fn from_value(value: &Value) -> Result<Self, CustomError> {
        Ok(value.to_string())
    }
}
