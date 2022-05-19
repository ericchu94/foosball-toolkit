use serde::{Deserialize, Deserializer, Serialize, Serializer};
use time::{format_description::FormatItem, macros::format_description, PrimitiveDateTime};

const FORMAT: &[FormatItem] = format_description!("[day]/[month]/[year] [hour]:[minute]:[second]");

pub fn serialize<S>(value: &Option<PrimitiveDateTime>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    #[derive(Serialize)]
    struct Helper<'a>(#[serde(with = "super::fast_date_time")] &'a PrimitiveDateTime);

    value
        .as_ref()
        .map(|date| date.format(FORMAT).unwrap())
        .serialize(serializer)
}

// The signature of a deserialize_with function must follow the pattern:
//
//    fn deserialize<'de, D>(D) -> Result<T, D::Error>
//    where
//        D: Deserializer<'de>
//
// although it may also be generic over the output types T.
pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<PrimitiveDateTime>, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    struct Helper(#[serde(with = "super::fast_date_time")] PrimitiveDateTime);

    let helper = Option::<String>::deserialize(deserializer)?;
    helper.map_or(Ok(None), |s| {
        PrimitiveDateTime::parse(&s, FORMAT)
            .map_err(serde::de::Error::custom)
            .map(Some)
    })
}
