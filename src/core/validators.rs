use time::{PrimitiveDateTime, format_description::well_known::Iso8601};
use validator::ValidationError;

pub fn validate_datetime(value: &str) -> Result<(), ValidationError> {
    PrimitiveDateTime::parse(value, &Iso8601::DEFAULT)
        .map(|_| ())
        .map_err(|_| ValidationError::new("invalid_datetime"))
}
