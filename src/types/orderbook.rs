use crate::{BidAskCrossError, Book, ConditionId, TimestampVisitor, TokenId};
use derive_more::{From, Into};
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

#[derive(From, Into, Serialize, Deserialize, Eq, PartialEq, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct Orderbook {
    pub market: ConditionId,
    pub asset_id: TokenId,
    pub hash: String,
    #[serde(deserialize_with = "TimestampVisitor::deserialize")]
    pub timestamp: OffsetDateTime,
    pub bids: Book,
    pub asks: Book,
}

impl Orderbook {
    pub fn validate(&self) -> Result<(), BidAskCrossError> {
        let max_bid_price_opt = self.bids.keys().max();
        let min_ask_price_opt = self.asks.keys().min();
        match (max_bid_price_opt, min_ask_price_opt) {
            (Some(max_bid_price), Some(min_ask_price)) if max_bid_price >= min_ask_price => Err(BidAskCrossError::new(*max_bid_price, *min_ask_price)),
            _ => Ok(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn must_deserialize() {
        let input = include_str!("../../fixtures/orderbook.json");
        let orderbook: Orderbook = serde_json::de::from_str(input).unwrap();
        assert_eq!(orderbook.hash, "6b57f28fe93242322f8836463d3266551166f90b")
    }
}
