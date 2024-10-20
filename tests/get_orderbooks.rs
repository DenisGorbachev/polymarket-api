use polymarket_api::{RestClient, TokenId, NEXT_CURSOR_START};
use std::env;

mod common;

#[tokio::test]
#[ignore]
async fn test_orderbooks() {
    env_logger::init();
    let next_cursor = env::var("NEXT_CURSOR").unwrap_or(NEXT_CURSOR_START.into());
    let client = RestClient::default();
    let markets_payload = client
        .get_markets(next_cursor)
        .await
        .expect("should get markets");
    let markets = markets_payload.data;
    let token_ids = markets
        .iter()
        .flat_map(|market| market.tokens.token_ids_vec())
        .collect::<Vec<TokenId>>();
    let orderbooks = client.get_orderbooks(&token_ids).await.unwrap();
    dbg!(&orderbooks);
    assert_eq!(orderbooks.len(), markets.len() * 2);
}

// pub fn collect_all() ->  {
//     todo!()
// }
