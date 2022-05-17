use serde::{Deserialize, Deserializer, Serializer};
use time::{format_description::FormatItem, macros::format_description, Date};

const FORMAT: &[FormatItem] = format_description!("[day]/[month]/[year]");

pub fn serialize<S>(date: &Date, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let s = date.format(FORMAT).unwrap();
    serializer.serialize_str(&s)
}

// The signature of a deserialize_with function must follow the pattern:
//
//    fn deserialize<'de, D>(D) -> Result<T, D::Error>
//    where
//        D: Deserializer<'de>
//
// although it may also be generic over the output types T.
pub fn deserialize<'de, D>(deserializer: D) -> Result<Date, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    Date::parse(&s, FORMAT).map_err(serde::de::Error::custom)
}
