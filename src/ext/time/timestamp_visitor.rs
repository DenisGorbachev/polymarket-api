use serde::de::{Error, Visitor};
use serde::{Deserializer, Serialize, Serializer};
use std::fmt;
use time::OffsetDateTime;

pub struct TimestampVisitor;

impl Visitor<'_> for TimestampVisitor {
    type Value = OffsetDateTime;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a string containing a u64 timestamp in milliseconds")
    }

    fn visit_str<E: Error>(self, value: &str) -> Result<Self::Value, E> {
        let timestamp_ms = value.parse::<u64>().map_err(E::custom)?;
        OffsetDateTime::from_unix_timestamp_nanos((timestamp_ms as i128) * 1_000_000).map_err(E::custom)
    }
}

impl TimestampVisitor {
    pub fn serialize<S: Serializer>(datetime: &OffsetDateTime, serializer: S) -> Result<S::Ok, S::Error> {
        let timestamp = datetime.unix_timestamp_nanos() / 1_000_000;
        timestamp.to_string().serialize(serializer)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<OffsetDateTime, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(TimestampVisitor)
    }
}
