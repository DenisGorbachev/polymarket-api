use crate::{Market, Payload};
use derive_getters::Getters;
use derive_more::{From, Into};
use derive_new::new;
use serde::de::DeserializeOwned;
use url::Url;

#[derive(new, Getters, From, Into, Ord, PartialOrd, Eq, PartialEq, Hash, Clone, Debug)]
pub struct Client {
    base_url: Url,
}

impl Client {
    pub fn url(&self, path: &str) -> Url {
        let mut url = self.base_url.clone();
        url.set_path(path);
        url
    }

    pub async fn get_markets(&self) -> reqwest::Result<Vec<Market>> {
        let payload = self.get_payload::<Market>("/markets").await?;
        Ok(payload.data)
    }

    async fn get_payload<T: DeserializeOwned>(&self, path: &str) -> reqwest::Result<Payload<T>> {
        let response = reqwest::get(self.url(path)).await?;
        let data = response.json().await?;
        Ok(data)
    }
}
