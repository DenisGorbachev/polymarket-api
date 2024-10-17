use crate::Token;
use derive_more::{From, Into};
use derive_new::new;
use serde::{Deserialize, Serialize};

/// See [`Tokens::winner`]
#[derive(new, From, Into, Serialize, Deserialize, Ord, PartialOrd, Eq, PartialEq, Default, Hash, Clone, Debug)]
pub struct Tokens {
    pub left: Token,
    pub right: Token,
}

impl Tokens {
    /// The docs don't specify whether `self.left.winner` and `self.right.winner` can be `true` at the same time
    pub fn winner(&self) -> Option<Option<&Token>> {
        match (self.left.winner, self.right.winner) {
            (true, true) => None,
            (false, false) => Some(None),
            (true, false) => Some(Some(&self.left)),
            (false, true) => Some(Some(&self.right)),
        }
    }
}
