use rust_decimal::Decimal;
use serde::de::{self, Deserializer, Visitor};
use serde::ser::Serializer;
use std::fmt;

struct DecimalVisitor;

impl<'de> Visitor<'de> for DecimalVisitor {
    type Value = Decimal;

    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("a JSON number or decimal string")
    }

    fn visit_f64<E: de::Error>(self, v: f64) -> Result<Decimal, E> {
        let s = v.to_string();
        s.parse::<Decimal>().map_err(de::Error::custom)
    }

    fn visit_i64<E: de::Error>(self, v: i64) -> Result<Decimal, E> {
        Ok(Decimal::from(v))
    }

    fn visit_u64<E: de::Error>(self, v: u64) -> Result<Decimal, E> {
        Ok(Decimal::from(v))
    }

    fn visit_str<E: de::Error>(self, v: &str) -> Result<Decimal, E> {
        v.parse::<Decimal>().map_err(de::Error::custom)
    }
}

pub fn deserialize<'de, D>(deserializer: D) -> Result<Decimal, D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_any(DecimalVisitor)
}

pub fn serialize<S>(value: &Decimal, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    rust_decimal::serde::float::serialize(value, serializer)
}

pub mod opt {
    use super::*;

    struct OptDecimalVisitor;

    impl<'de> Visitor<'de> for OptDecimalVisitor {
        type Value = Option<Decimal>;

        fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
            f.write_str("a JSON number, decimal string, or null")
        }

        fn visit_none<E: de::Error>(self) -> Result<Option<Decimal>, E> {
            Ok(None)
        }

        fn visit_unit<E: de::Error>(self) -> Result<Option<Decimal>, E> {
            Ok(None)
        }

        fn visit_f64<E: de::Error>(self, v: f64) -> Result<Option<Decimal>, E> {
            let s = v.to_string();
            s.parse::<Decimal>().map(Some).map_err(de::Error::custom)
        }

        fn visit_i64<E: de::Error>(self, v: i64) -> Result<Option<Decimal>, E> {
            Ok(Some(Decimal::from(v)))
        }

        fn visit_u64<E: de::Error>(self, v: u64) -> Result<Option<Decimal>, E> {
            Ok(Some(Decimal::from(v)))
        }

        fn visit_str<E: de::Error>(self, v: &str) -> Result<Option<Decimal>, E> {
            v.parse::<Decimal>().map(Some).map_err(de::Error::custom)
        }
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<Decimal>, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(OptDecimalVisitor)
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
