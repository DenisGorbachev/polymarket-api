use derive_more::{From, Into};
use derive_new::new;
use serde::{Deserialize, Serialize};

#[derive(new, From, Into, Serialize, Deserialize, Ord, PartialOrd, Eq, PartialEq, Default, Hash, Clone, Debug)]
pub struct Payload<T> {
    pub limit: u64,
    pub count: u64,
    pub next_cursor: String,
    pub data: Vec<T>,
}

impl<T> Payload<T> {}
