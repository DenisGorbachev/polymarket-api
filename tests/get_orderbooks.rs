use futures::{StreamExt, TryStreamExt};
use polymarket_api::{Market, RestClient, TokenId};
use std::future::ready;
use tokio::pin;

mod common;

// TODO: Migrate to GammaClient + get markets that are guaranteed to be active
#[tokio::test]
async fn test_orderbooks() {
    env_logger::init();
    let client = RestClient::default();
    let markets_stream_raw = client.get_markets_stream();
    let markets_stream_filtered = markets_stream_raw.try_filter_map(|markets| {
        let markets = markets
            .into_iter()
            .filter(|m| m.enable_order_book)
            .collect::<Vec<Market>>();
        let output = if markets.is_empty() { None } else { Some(markets) };
        ready(Ok(output))
    });
    pin!(markets_stream_filtered);
    let markets = markets_stream_filtered
        .next()
        .await
        .expect("must return some markets with enable_order_book = true")
        .unwrap();
    assert!(!markets.is_empty(), "expecting markets.len() > 0, otherwise we're not really testing the orderbooks");
    let token_ids = markets
        .iter()
        .flat_map(|market| market.tokens.token_ids_vec())
        .collect::<Vec<TokenId>>();
    let orderbooks = client.get_orderbooks(&token_ids).await.unwrap();
    assert_eq!(orderbooks.len(), markets.len() * 2);
}
