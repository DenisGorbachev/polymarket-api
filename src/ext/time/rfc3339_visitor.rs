use serde::de::{Error, Visitor};
use serde::Deserializer;
use std::fmt;
use time::format_description::well_known::Rfc3339;
use time::OffsetDateTime;

pub struct Rfc3339Visitor;

impl Visitor<'_> for Rfc3339Visitor {
    type Value = OffsetDateTime;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a string in RFC 3339 format")
    }

    fn visit_str<E: Error>(self, value: &str) -> Result<Self::Value, E> {
        OffsetDateTime::parse(value, &Rfc3339).map_err(|e| E::custom(format!("Failed to parse RFC 3339 date: {}", e)))
    }
}

impl Rfc3339Visitor {
    pub fn deserialize<'de, D>(deserializer: D) -> Result<OffsetDateTime, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(Rfc3339Visitor)
    }
}
