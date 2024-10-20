use derive_more::From;
use serde::{Deserialize, Serialize};

#[derive(From, Serialize, Deserialize, Ord, PartialOrd, Eq, PartialEq, Hash, Clone, Copy, Debug)]
pub enum OrderType {
    #[serde(rename = "GTC")]
    GoodTillCancelled,
    #[serde(rename = "FOK")]
    FillOrKill,
    #[serde(rename = "GTD")]
    GoodTillDay,
}

impl OrderType {}
