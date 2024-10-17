use crate::{Amount, Rewards, Tokens};
use alloy::primitives::{Address, B256};
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
    pub condition_id: B256,
    pub question_id: B256,
    pub active: bool,
    pub closed: bool,
    pub archived: bool,
    pub enable_order_book: bool,
    pub accepting_orders: bool,
    pub accepting_order_timestamp: Option<OffsetDateTime>,
    pub minimum_order_size: Amount,
    pub minimum_tick_size: Amount,
    pub end_date_iso: OffsetDateTime,
    pub game_start_time: Option<OffsetDateTime>,
    pub seconds_delay: Duration,
    pub fpmm: Option<Address>,
    pub maker_base_fee: Amount,
    pub taker_base_fee: Amount,
    pub neg_risk: bool,
    pub neg_risk_market_id: String,
    pub neg_risk_request_id: String,
    pub rewards: Rewards,
    pub tokens: Tokens,
    pub is_50_50_outcome: bool,
    pub notifications_enabled: bool,
    pub tags: Vec<String>,
}

impl Market {}
