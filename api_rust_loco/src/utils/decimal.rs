use rust_decimal::Decimal;
use serde::de::{self, Deserialize, Deserializer, Unexpected};
use serde::ser::Serializer;
use serde_json::value::Number;

pub fn deserialize<'de, D>(deserializer: D) -> Result<Decimal, D::Error>
where
    D: Deserializer<'de>,
{
    let num = Number::deserialize(deserializer)?;
    let s = num.to_string();
    s.parse::<Decimal>()
        .map_err(|_| de::Error::invalid_value(Unexpected::Str(&s), &"a decimal number"))
}

pub fn serialize<S>(value: &Decimal, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    rust_decimal::serde::float::serialize(value, serializer)
}
