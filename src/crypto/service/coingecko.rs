use super::CryptoService;
use crate::crypto::{CryptImageURL, Crypto, CryptoDevInfo};
use async_trait::async_trait;
use std::collections::HashMap;

#[derive(Debug)]
pub struct CoingeckoService {
    name: String,
    client: reqwest::Client,
    base_url: String,
}

impl CoingeckoService {
    /// Creates a new [`CoingeckoService`].
    pub fn new() -> CoingeckoService {
        CoingeckoService {
            name: String::from("Coingecko"),
            base_url: String::from("https://api.coingecko.com"),
            client: reqwest::Client::new(),
        }
    }
}

#[async_trait]
impl CryptoService for CoingeckoService {
    async fn cryptos(&self) -> Result<Vec<Crypto>, Box<dyn std::error::Error>> {
        let url = format!("{}/api/v3/coins/list", self.base_url);
        let basic_crypto = self
            .client
            .get(url)
            .send()
            .await?
            .json::<Vec<Crypto>>()
            .await?;

        let basic_crypto = basic_crypto
            .iter()
            .map(|c| {
                let mut crypto = c.to_owned();
                crypto.platform = Some(self.name.clone());
                crypto
            })
            .collect::<Vec<Crypto>>();
        Ok(basic_crypto)
    }

    async fn conversion(
        &self,
        amount: f64,
        base_id: &str,
        quote_id: &str,
    ) -> Result<f64, Box<dyn std::error::Error>> {
        let url = format!("{}/api/v3/simple/price", self.base_url);
        let mut params = HashMap::new();
        params.insert("ids", base_id.to_owned());
        params.insert("vs_currencies", quote_id.to_owned());
        let request_builder = self
            .client
            .get(url)
            .query(&params)
            .send()
            .await?
            .json::<HashMap<String, HashMap<String, f64>>>()
            .await?;
        let conversion_rate = request_builder.get(base_id).unwrap().get(quote_id).unwrap();
        Ok(amount * conversion_rate)
    }

    async fn image(&self, id: &str) -> Result<CryptImageURL, Box<dyn std::error::Error>> {
        let url = format!("{}/api/v3/coins/{}?localization=false&tickers=false&market_data=false&community_data=false&developer_data=true&sparkline=false", self.base_url, id);
        let resp = self
            .client
            .get(url)
            .send()
            .await?
            .json::<CryptoDevInfo>()
            .await?;
        Ok(resp.image)
    }

    async fn get_supported_quoted_currency(
        &self,
    ) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let url = format!("{}/api/v3/simple/supported_vs_currencies", self.base_url);

        let resp = self
            .client
            .get(url)
            .send()
            .await?
            .json::<Vec<String>>()
            .await?;

        Ok(resp)
    }
}
