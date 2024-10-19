use crate::{Orderbook, Payload, REST_BASE_URL};
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

    pub async fn get_markets(&self) -> reqwest::Result<Vec<Orderbook>> {
        let payload = self.get_payload::<Orderbook>("/markets").await?;
        Ok(payload.data)
    }

    async fn get_payload<T: DeserializeOwned>(&self, path: &str) -> reqwest::Result<Payload<T>> {
        let response = reqwest::get(self.url(path)).await?;
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
