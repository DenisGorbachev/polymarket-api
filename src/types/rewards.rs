use crate::{Amount, RewardsRate, RewardsRaw};
use derive_more::{From, Into};
use derive_new::new;
use serde::{Deserialize, Serialize};

#[derive(new, From, Into, Serialize, Deserialize, Ord, PartialOrd, Eq, PartialEq, Default, Hash, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct Rewards {
    pub rates: Vec<RewardsRate>,
    pub min_size: Amount,
    pub max_spread: Amount,
}

impl Rewards {}

impl TryFrom<RewardsRaw> for Rewards {
    type Error = ();

    fn try_from(value: RewardsRaw) -> Result<Self, Self::Error> {
        let RewardsRaw {
            rates,
            min_size,
            max_spread,
        } = value;
        let rates = rates.unwrap_or_default();
        Ok(Self {
            rates,
            min_size,
            max_spread,
        })
    }
}
