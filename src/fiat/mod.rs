
use async_trait::async_trait;
use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq)]
#[allow(dead_code)]
pub struct Fiat {
    id: Option<String>,
    name: String,
    symbol: String,
}

#[async_trait]
pub trait FiatService {
    /// Converts an amount from one currency to another.
    /// - base: The currency to convert from.
    /// - quote: The currency to convert to.
    /// - amount: The amount to convert.
    async fn conversion(
        &self,
        amount: &f64,
        base: &'_ Fiat,
        quote: &'_ Fiat,
    ) -> Result<f64, Box<dyn std::error::Error>>;
    /// Returns a list of all supported currencies.
    async fn get_all_supported_fiat(&self) -> Result<Vec<Fiat>, Box<dyn std::error::Error>>;
}

impl Fiat {
    pub fn new(name: String, symbol: String) -> Self {
        Fiat { id:None, name, symbol }
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
    pub async fn conversion(
        &self,
        amount: &f64,
        convert_to: &Fiat,
        service: &dyn FiatService,
    ) -> Result<f64, Box<dyn std::error::Error>> {
        let result = service
            .conversion(amount, &self, &convert_to)
            .await;
        result
    }
}