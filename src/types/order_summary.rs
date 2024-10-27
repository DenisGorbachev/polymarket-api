use crate::{Amount, Price};
use derive_more::{From, Into};
use serde::{Deserialize, Serialize};

#[derive(From, Into, Serialize, Deserialize, Ord, PartialOrd, Eq, PartialEq, Hash, Clone, Copy, Debug)]
#[serde(deny_unknown_fields)]
pub struct OrderSummary {
    pub price: Amount,
    pub size: Amount,
}

impl OrderSummary {}

impl From<(&Price, &Amount)> for OrderSummary {
    fn from((price, size): (&Price, &Amount)) -> Self {
        Self {
            price: *price,
            size: *size,
        }
    }
}
