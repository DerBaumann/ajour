use serde::{Deserialize, Deserializer};

pub fn blank_as_none<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: Deserializer<'de>,
{
    let opt = Option::<String>::deserialize(deserializer)?;
    Ok(opt.filter(|v| !v.is_empty()))
}
