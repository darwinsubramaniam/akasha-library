pub mod service;
use async_trait::async_trait;

#[derive(Debug)]
#[allow(dead_code)]
pub struct Fiat {
    name: String,
    symbol: String,
}

#[async_trait]
pub trait FiatService {
    /// Returns a list of all supported currencies.
    async fn currencies(&self) -> Result<Vec<Fiat>, Box<dyn std::error::Error>>;
    /// Converts an amount from one currency to another.
    /// - base: The currency to convert from.
    /// - quote: The currency to convert to.
    /// - amount: The amount to convert.
    async fn conversion(
        &self,
        amount: f64,
        base: &str,
        quote: &str,
    ) -> Result<f64, Box<dyn std::error::Error>>;
}

impl Fiat {
    pub fn new(name: String, symbol: String) -> Self {
        Fiat { name, symbol }
    }

    /// Get the fullname of the fiat
    pub fn name(&self) -> &str {
        &self.name
    }
    /// Get the symbol of the fiat
    pub fn symbol(&self) -> &str {
        &self.symbol
    }

    /// perform the conversion on the fiat against another fiat
    pub async fn conversion<'a>(
        &self,
        amount: f64,
        convert_to: &Fiat,
        service: Box<dyn FiatService + 'a>,
    ) -> Result<f64, Box<dyn std::error::Error>> {
        let result = service
            .conversion(amount, &self.symbol, &convert_to.symbol)
            .await;
        result
    }
}
