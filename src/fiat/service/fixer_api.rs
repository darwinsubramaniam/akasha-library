use async_trait::async_trait;
use reqwest::RequestBuilder;
use serde::Deserialize;
use std::collections::HashMap;

use crate::fiat::{Fiat, FiatService};

#[derive(Debug)]
#[allow(dead_code)]
pub struct FixerApiService<'a> {
    apikey: &'a str,
    base_url: &'a str,
    client: reqwest::Client,
}

#[async_trait]
impl FiatService for FixerApiService<'_> {
    async fn currencies(&self) -> Result<Vec<Fiat>, Box<dyn std::error::Error>> {
        let url = format!("{}/fixer/symbols", self.base_url);
        dbg!(format!("url: {}", url));
        let resp = self.fetch::<FixerApiSymbol>(self.client.get(url)).await?;
        Ok(resp.to_currency_naming())
    }

    async fn conversion(
        &self,
        amount: f64,
        base: &str,
        quote: &str,
    ) -> Result<f64, Box<dyn std::error::Error>> {
        let url = format!("{}/fixer/convert", self.base_url);

        let mut params = HashMap::new();
        params.insert("from", base.to_owned());
        params.insert("to", quote.to_owned());
        params.insert("amount", amount.to_string());

        let request_builder = self.client.get(url).query(&params);
        let resp = self.fetch::<FixerApiConvert>(request_builder).await?;

        Ok(resp.result)
    }
}

impl FixerApiService<'_> {
    /// Creates a new [`FixerApiService`].
    #[allow(dead_code)]
    pub fn new(apikey: &str) -> FixerApiService {
        FixerApiService {
            apikey,
            base_url: "https://api.apilayer.com",
            client: reqwest::Client::new(),
        }
    }

    /// attach the api key to the request header and send the request.
    /// ## Errors
    /// This function will return an error the request status is not 200-299.
    async fn fetch<T: for<'de> Deserialize<'de>>(
        &self,
        request_builder: RequestBuilder,
    ) -> Result<T, Box<dyn std::error::Error>> {
        let resp = request_builder
            .header("apikey", self.apikey)
            .send()
            .await?;

        match resp.status().is_success() {
            true => {
                let resp = resp.json::<T>().await?;
                Ok(resp)
            }
            false => Err(format!("Request failed: {}", resp.status()).into()),
        }
    }
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct FixerApiSymbol {
    success: bool,
    /// Symbol : Fullname of the currency
    /// ### Entry example:
    /// | Symbol | Fullname |
    /// |---|---|
    /// |"USD"| "United States Dollar"|
    /// |"EUR"| "Euro"|
    #[serde(rename = "symbols")]
    pub symbol_fullname: HashMap<String, String>,
}

impl FixerApiSymbol {
    /// Converts the [`FixerApiSymbol`] to a [`Vec<Fiat>`].
    pub fn to_currency_naming(&self) -> Vec<Fiat> {
        self.symbol_fullname
            .iter()
            .map(|(symbol, name)| Fiat {
                symbol: symbol.to_string(),
                name: name.to_string(),
            })
            .collect()
    }
}

/// The response of the fixer api convert endpoint.
#[derive(Debug, Deserialize)]
pub struct FixerApiConvert {
    pub date: String,
    pub info: FixerApiConvertInfo,
    pub query: FixerApiConvertQuery,
    #[serde(rename = "result")]
    pub result: f64,
    #[serde(rename = "success")]
    pub is_success: bool,
}

#[derive(Debug, Deserialize)]
pub struct FixerApiConvertInfo {
    pub rate: f64,
    pub timestamp: i64,
}

#[derive(Debug, Deserialize)]
pub struct FixerApiConvertQuery {
    pub amount: f64,
    pub from: String,
    pub to: String,
}
