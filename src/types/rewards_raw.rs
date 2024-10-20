use crate::{Amount, RewardsRate};
use derive_more::{From, Into};
use derive_new::new;
use serde::{Deserialize, Serialize};

#[derive(new, From, Into, Serialize, Deserialize, Ord, PartialOrd, Eq, PartialEq, Default, Hash, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct RewardsRaw {
    pub rates: Option<Vec<RewardsRate>>,
    pub min_size: Amount,
    pub max_spread: Amount,
}

impl RewardsRaw {}
