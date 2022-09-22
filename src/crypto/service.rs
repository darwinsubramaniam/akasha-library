use async_trait::async_trait;

use super::model::{CryptImageURL, Crypto};

#[async_trait]
pub trait CryptoInformationFetcher {
    /// Returns a list of all supported crypto.
    async fn cryptos(&self) -> Result<Vec<Crypto>, Box<dyn std::error::Error>>;
    /// Converts an amount from one crypto to another.
    /// - base: The crypto to convert from.
    /// - quote: The crypto to convert to.
    /// - amount: The amount to convert.
    async fn conversion(
        &self,
        amount: f64,
        base_id: &str,
        quote_id: &str,
    ) -> Result<f64, Box<dyn std::error::Error>>;

    /// Get the image from using the id defined in the platform where the crypto was fetched
    async fn image(
        &self,
        platform_defined_id: &str,
    ) -> Result<CryptImageURL, Box<dyn std::error::Error>>;

    ///Get the list of supported crypto or currency for the conversion rate.
    async fn get_supported_quoted_currency(
        &self,
    ) -> Result<Vec<String>, Box<dyn std::error::Error>>;
}
