use crate::{get_page_stream, Market, MarketRaw, NextCursor, Orderbook, Payload, TokenId, NEXT_CURSOR_START, REST_BASE_URL};
use derive_getters::Getters;
use derive_more::{From, Into};
use derive_new::new;
use futures::Stream;
use serde::de::DeserializeOwned;
use std::fs::File;
use std::io::Write;
use url::Url;

#[derive(new, Getters, From, Into, Ord, PartialOrd, Eq, PartialEq, Hash, Clone, Debug)]
pub struct RestClient {
    base_url: Url,
}

impl RestClient {
    pub fn url(&self, path: &str) -> Url {
        let mut url = self.base_url.clone();
        url.set_path(path);
        url
    }

    pub async fn get_markets(&self, next_cursor: NextCursor) -> reqwest::Result<Payload<Market>> {
        let result = self
            .get_payload::<MarketRaw>(self.url("/markets"), next_cursor)
            .await;
        result.map(|payload| {
            let Payload {
                limit,
                count,
                next_cursor,
                data,
            } = payload;
            let data = data
                .into_iter()
                .map(Market::try_from)
                .filter_map(Result::ok)
                .collect::<Vec<Market>>();
            Payload {
                limit,
                count,
                next_cursor,
                data,
            }
        })
    }

    /// Retrieves the orderbooks for the specified token IDs.
    ///
    /// # Arguments
    /// * `token_ids` - An iterator of token IDs to retrieve orderbooks for.
    ///
    /// # Returns
    /// A `reqwest::Result<Vec<Orderbook>>` containing the orderbooks for the specified token IDs.
    pub async fn get_orderbooks(&self, _token_ids: impl IntoIterator<Item = &TokenId>) -> reqwest::Result<Vec<Orderbook>> {
        todo!()
        // let result = self
        //     .get_payload::<MarketRaw>(self.url("/markets"), next_cursor)
        //     .await;
    }

    pub fn get_markets_stream(&self) -> impl Stream<Item = Result<Vec<Market>, reqwest::Error>> + '_ {
        self.get_markets_stream_at_cursor(NEXT_CURSOR_START.into())
    }

    pub fn get_markets_stream_at_cursor(&self, next_cursor: NextCursor) -> impl Stream<Item = Result<Vec<Market>, reqwest::Error>> + '_ {
        get_page_stream(|next_cursor| self.get_markets(next_cursor), next_cursor)
    }

    async fn get_payload<T: DeserializeOwned>(&self, mut url: Url, next_cursor: NextCursor) -> reqwest::Result<Payload<T>> {
        url.query_pairs_mut()
            .append_pair("next_cursor", &next_cursor);
        let response = reqwest::get(url).await?;
        let full = response.text().await?;
        File::options()
            .create(true)
            .truncate(true)
            .write(true)
            .open("/tmp/payload.json")
            .unwrap()
            .write_all(full.as_bytes())
            .unwrap();
        let data = serde_json::from_slice(full.as_bytes()).unwrap();
        // let data = response.json().await?;
        Ok(data)
    }
}

impl Default for RestClient {
    fn default() -> Self {
        Self {
            base_url: REST_BASE_URL.clone(),
        }
    }
}
