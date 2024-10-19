use crate::{Amount, TokenId};
use derive_more::{From, Into};
use derive_new::new;
use serde::{Deserialize, Serialize};

#[derive(new, From, Into, Serialize, Deserialize, Ord, PartialOrd, Eq, PartialEq, Default, Hash, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct Token {
    /// Examples: `"21742633143463906290569050155826241533067272736897614950488156847949938836455"`
    pub token_id: TokenId,
    /// Examples: `"Yes"`, `"No"`
    pub outcome: String,
    /// In nominal units of the quote currency (e.g. in USD)
    /// Examples: `0.5845`
    pub price: Amount,
    pub winner: bool,
}

impl Token {}
