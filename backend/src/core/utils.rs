use time::{PrimitiveDateTime, format_description};

pub fn parse_form_datetime(value: &str) -> Result<PrimitiveDateTime, time::Error> {
    let format = format_description::parse("[year]-[month]-[day] [hour]:[minute]")
        .expect("Should be a valid date format");
    Ok(PrimitiveDateTime::parse(value, &format)?)
}
