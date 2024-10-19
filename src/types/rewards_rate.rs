use crate::Amount;
use alloy::primitives::Address;
use derive_more::{From, Into};
use derive_new::new;
use serde::{Deserialize, Serialize};

#[derive(new, From, Into, Serialize, Deserialize, Ord, PartialOrd, Eq, PartialEq, Default, Hash, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct RewardsRate {
    pub asset_address: Address,
    pub rewards_daily_rate: Amount,
}

impl RewardsRate {}
