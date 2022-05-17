use serde::{Deserialize, Deserializer, Serializer};
use time::{format_description::FormatItem, macros::format_description, PrimitiveDateTime};

const FORMAT: &[FormatItem] = format_description!("[day]/[month]/[year] [hour]:[minute]:[second]");

pub fn serialize<S>(date: &PrimitiveDateTime, serializer: S) -> Result<S::Ok, S::Error>
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
pub fn deserialize<'de, D>(deserializer: D) -> Result<PrimitiveDateTime, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    PrimitiveDateTime::parse(&s, FORMAT).map_err(serde::de::Error::custom)
}
