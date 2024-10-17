use crate::Amount;
use alloy::primitives::U256;
use derive_more::{From, Into};
use derive_new::new;
use serde::{Deserialize, Serialize};

#[derive(new, From, Into, Serialize, Deserialize, Ord, PartialOrd, Eq, PartialEq, Default, Hash, Clone, Debug)]
pub struct Token {
    pub token_id: U256,
    /// For example: `"Yes"`, `"No"`
    pub outcome: String,
    pub price: Amount,
    pub winner: bool,
}

impl Token {}
