use derive_more::From;
use serde::{Deserialize, Serialize};

#[derive(From, Serialize, Deserialize, Ord, PartialOrd, Eq, PartialEq, Hash, Clone, Copy, Debug)]
#[serde(rename_all = "UPPERCASE")]
pub enum Side {
    Buy,
    Sell,
}

impl Side {}
