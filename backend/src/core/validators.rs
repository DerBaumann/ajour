use validator::ValidationError;

use crate::core::utils::{parse_form_date, parse_form_datetime};

pub fn validate_datetime(value: &str) -> Result<(), ValidationError> {
    parse_form_datetime(value).map(|_| ()).map_err(|e| {
        tracing::error!(error=?e, "Datetime parsing error");
        ValidationError::new("invalid_datetime")
    })
}

pub fn validate_date(value: &str) -> Result<(), ValidationError> {
    parse_form_date(value).map(|_| ()).map_err(|e| {
        tracing::error!(error=?e, "Date parsing error");
        ValidationError::new("invalid_date")
    })
}
