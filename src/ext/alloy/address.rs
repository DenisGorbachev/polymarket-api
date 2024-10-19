use alloy::primitives::Address;
use std::str::FromStr;

pub fn deserialize_address_or_empty_string<'de, D>(deserializer: D) -> Result<Option<Address>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s: String = serde::Deserialize::deserialize(deserializer)?;
    if s.is_empty() {
        Ok(None)
    } else {
        Address::from_str(&s)
            .map(Some)
            .map_err(serde::de::Error::custom)
    }
}
