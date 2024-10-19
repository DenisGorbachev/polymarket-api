use std::sync::LazyLock;
use url::Url;
use url_macro::url;

pub static REST_BASE_URL: LazyLock<Url> = LazyLock::new(|| url!("https://clob.polymarket.com"));
pub static WS_BASE_URL: LazyLock<Url> = LazyLock::new(|| url!("wss://ws-subscriptions-clob.polymarket.com/ws"));

pub const YES: &str = "Yes";
pub const NO: &str = "No";
