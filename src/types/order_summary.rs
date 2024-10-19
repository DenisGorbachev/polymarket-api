use crate::Amount;
use derive_more::{From, Into};
use serde::{Deserialize, Serialize};

#[derive(From, Into, Serialize, Deserialize, Ord, PartialOrd, Eq, PartialEq, Hash, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct OrderSummary {
    pub price: Amount,
    pub size: Amount,
}

impl OrderSummary {}
