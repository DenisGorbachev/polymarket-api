use crate::{Amount, OrderSummary, Price};
use derive_more::{From, Into};
use indexmap::IndexMap;
use rustc_hash::FxBuildHasher;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use subtype::subtype;

subtype!(
    #[derive(PartialEq, Eq, Clone, Debug)]
    pub struct Book(IndexMap<Price, Amount, FxBuildHasher>)
);

impl Book {
    pub fn min(&self) -> Option<OrderSummary> {
        self.iter().min_by_key(|x| x.0).map(OrderSummary::from)
    }

    pub fn max(&self) -> Option<OrderSummary> {
        self.iter().max_by_key(|x| x.0).map(OrderSummary::from)
    }
}

impl Serialize for Book {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let order_summaries: Vec<OrderSummary> = self.iter().map(OrderSummary::from).collect();
        order_summaries.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Book {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let order_summaries: Vec<OrderSummary> = Vec::deserialize(deserializer)?;
        let map = order_summaries
            .into_iter()
            .map(OrderSummary::into)
            .collect();
        Ok(Self(map))
    }
}
