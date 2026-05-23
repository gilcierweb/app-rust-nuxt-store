use rust_decimal::Decimal;
use serde::de::{self, Deserialize, Deserializer};
use serde::ser::Serializer;
use serde_json::value::Number;

fn number_to_decimal(num: Number) -> Result<Decimal, String> {
    let s = num.to_string();
    s.parse::<Decimal>().map_err(|_| format!("invalid decimal: {s}"))
}

pub fn deserialize<'de, D>(deserializer: D) -> Result<Decimal, D::Error>
where
    D: Deserializer<'de>,
{
    let num = Number::deserialize(deserializer)?;
    number_to_decimal(num).map_err(de::Error::custom)
}

pub fn serialize<S>(value: &Decimal, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    rust_decimal::serde::float::serialize(value, serializer)
}

pub mod opt {
    use super::*;

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<Decimal>, D::Error>
    where
        D: Deserializer<'de>,
    {
        match Option::<Number>::deserialize(deserializer)? {
            Some(num) => number_to_decimal(num).map(Some).map_err(de::Error::custom),
            None => Ok(None),
        }
    }

    pub fn serialize<S>(value: &Option<Decimal>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match value {
            Some(v) => rust_decimal::serde::float::serialize(v, serializer),
            None => serializer.serialize_none(),
        }
    }
}
