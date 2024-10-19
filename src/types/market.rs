use crate::{deserialize_address_or_empty_string, Amount, ConditionId, DurationInSecondsVisitor, QuestionId, Rewards, Rfc3339Visitor, Tokens};
use alloy::primitives::Address;
use derive_more::{From, Into};
use serde::{Deserialize, Serialize};
use time::{Duration, OffsetDateTime};

#[derive(From, Into, Serialize, Deserialize, Ord, PartialOrd, Eq, PartialEq, Hash, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct Market {
    pub question: String,
    pub description: String,
    pub market_slug: String,
    pub icon: String,
    pub image: String,
    pub condition_id: ConditionId,
    pub question_id: QuestionId,
    pub active: bool,
    pub closed: bool,
    pub archived: bool,
    pub enable_order_book: bool,
    pub accepting_orders: bool,
    pub accepting_order_timestamp: Option<OffsetDateTime>,
    pub minimum_order_size: Amount,
    pub minimum_tick_size: Amount,
    #[serde(deserialize_with = "Rfc3339Visitor::deserialize")]
    pub end_date_iso: OffsetDateTime,
    pub game_start_time: Option<OffsetDateTime>,
    #[serde(deserialize_with = "DurationInSecondsVisitor::deserialize")]
    pub seconds_delay: Duration,
    #[serde(deserialize_with = "deserialize_address_or_empty_string")]
    pub fpmm: Option<Address>,
    pub maker_base_fee: Amount,
    pub taker_base_fee: Amount,
    pub neg_risk: bool,
    /// Confirmed that it's a `QuestionId`
    pub neg_risk_market_id: QuestionId,
    /// IMPORTANT: not sure if it's actually a `ConditionId`
    pub neg_risk_request_id: ConditionId,
    pub rewards: Rewards,
    pub tokens: Tokens,
    pub is_50_50_outcome: bool,
    pub notifications_enabled: bool,
    pub tags: Vec<String>,
}

impl Market {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn must_deserialize() {
        let input = include_str!("../../fixtures/market.json");
        let market: Market = serde_json::de::from_str(input).unwrap();
        assert_eq!(market.question, "Will Donald Trump win the 2024 US Presidential Election?")
    }
}
