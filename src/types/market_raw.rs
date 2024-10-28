use crate::{Amount, ConditionId, DeserializeOrNone, DurationInSecondsVisitor, QuestionId, RewardsRaw, Tokens};
use alloy::primitives::Address;
use derive_more::{From, Into};
use serde::{Deserialize, Serialize};
use time::{Duration, OffsetDateTime};

#[derive(From, Into, Serialize, Deserialize, Ord, PartialOrd, Eq, PartialEq, Hash, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct MarketRaw {
    pub question: String,
    pub description: String,
    pub market_slug: String,
    pub icon: String,
    pub image: String,
    /// TODO: It's very rare to see a market with empty `condition_id`; we need to simply filter them out
    #[serde(deserialize_with = "DeserializeOrNone::run")]
    pub condition_id: Option<ConditionId>,
    #[serde(deserialize_with = "DeserializeOrNone::run")]
    pub question_id: Option<QuestionId>,
    pub active: bool,
    pub closed: bool,
    pub archived: bool,
    pub enable_order_book: bool,
    pub accepting_orders: bool,
    #[serde(with = "time::serde::rfc3339::option")]
    pub accepting_order_timestamp: Option<OffsetDateTime>,
    pub minimum_order_size: Amount,
    pub minimum_tick_size: Amount,
    #[serde(with = "time::serde::rfc3339::option")]
    pub end_date_iso: Option<OffsetDateTime>,
    #[serde(with = "time::serde::rfc3339::option")]
    pub game_start_time: Option<OffsetDateTime>,
    #[serde(deserialize_with = "DurationInSecondsVisitor::deserialize")]
    pub seconds_delay: Duration,
    #[serde(deserialize_with = "DeserializeOrNone::run")]
    pub fpmm: Option<Address>,
    pub maker_base_fee: Amount,
    pub taker_base_fee: Amount,
    pub neg_risk: bool,
    /// Confirmed that it's a `QuestionId`
    /// TODO: Handle invalid QuestionId values (e.g. "0x12309") (some markets have an invalid `neg_risk_market_id` because they were created by Polymarket just for testing)
    #[serde(deserialize_with = "DeserializeOrNone::run")]
    pub neg_risk_market_id: Option<String>,
    /// IMPORTANT: not sure if it's actually a `ConditionId`
    #[serde(deserialize_with = "DeserializeOrNone::run")]
    pub neg_risk_request_id: Option<ConditionId>,
    pub rewards: RewardsRaw,
    pub tokens: Tokens,
    pub is_50_50_outcome: bool,
    pub notifications_enabled: bool,
    pub tags: Option<Vec<String>>,
}

impl MarketRaw {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn must_deserialize() {
        let input = include_str!("../../fixtures/market.json");
        let market: MarketRaw = serde_json::de::from_str(input).unwrap();
        assert_eq!(market.question, "Will Donald Trump win the 2024 US Presidential Election?")
    }
}
