use crate::{NextCursorRef, Orderbook, Payload, REST_BASE_URL};
use derive_getters::Getters;
use derive_more::{From, Into};
use derive_new::new;
use serde::de::DeserializeOwned;
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

    pub async fn get_markets(&self, next_cursor: &NextCursorRef) -> reqwest::Result<Payload<Orderbook>> {
        let url = self.url("/markets");
        self.get_payload::<Orderbook>(url, next_cursor).await
    }

    async fn get_payload<T: DeserializeOwned>(&self, mut url: Url, next_cursor: &NextCursorRef) -> reqwest::Result<Payload<T>> {
        url.query_pairs_mut()
            .append_pair("next_cursor", next_cursor);
        let response = reqwest::get(url).await?;
        let data = response.json().await?;
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
