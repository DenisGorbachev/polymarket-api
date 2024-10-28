use alloy::primitives::Uint;
use serde::{Deserialize, Deserializer, Serializer};

pub struct UintAsString;

impl UintAsString {
    pub fn serialize<const BITS: usize, const LIMBS: usize, S>(value: &Uint<BITS, LIMBS>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = value.to_string();
        serializer.serialize_str(&s)
    }

    pub fn deserialize<'de, const BITS: usize, const LIMBS: usize, D>(deserializer: D) -> Result<Uint<BITS, LIMBS>, D::Error>
    where
        D: Deserializer<'de>,
    {
        Uint::<BITS, LIMBS>::deserialize(deserializer)
    }
}
