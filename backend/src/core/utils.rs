use time::{Date, PrimitiveDateTime, format_description};

pub fn parse_form_datetime(value: &str) -> Result<PrimitiveDateTime, time::Error> {
    let format =
        format_description::parse("[day].[month].[year]").expect("Should be a valid date format");
    Ok(PrimitiveDateTime::parse(value, &format)?)
}

pub fn parse_form_date(value: &str) -> Result<Date, time::Error> {
    let format =
        format_description::parse("[day].[month].[year]").expect("Should be a valid date format");
    Ok(Date::parse(value, &format)?)
}
