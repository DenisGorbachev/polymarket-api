use crate::{Side, TokenId};
use derive_more::{From, Into};
use derive_new::new;
use serde::{Deserialize, Serialize};

#[derive(new, Serialize, Deserialize, From, Into, Ord, PartialOrd, Eq, PartialEq, Hash, Clone, Copy, Debug)]
pub struct BookParams {
    token_id: TokenId,
    #[serde(skip_serializing_if = "Option::is_none")]
    side: Option<Side>,
}

impl BookParams {}
