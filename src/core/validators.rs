use validator::ValidationError;

use crate::core::utils::parse_form_datetime;

pub fn validate_datetime(value: &str) -> Result<(), ValidationError> {
    parse_form_datetime(value)
        .map(|_| ())
        .map_err(|_| ValidationError::new("invalid_datetime"))
}
