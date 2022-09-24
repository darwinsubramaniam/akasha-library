use akasha::fiat::{FiatService, Fiat};

pub struct MockFiatService {}

impl Default for MockFiatService{
    fn default() -> Self {
        Self {  }
    }
}

#[async_trait::async_trait]
impl FiatService for MockFiatService {
    async fn conversion(
        &self,
        amount: &f64,
        base: &Fiat,
        quote: &Fiat,
    ) -> Result<f64, Box<dyn std::error::Error>> {
        if base.symbol() == "SGD" && quote.symbol() == "MYR"  {
            Ok(amount * 2.00)
        } else {
            Ok(amount / 2.00)
        }
    }

   async fn get_all_supported_fiat(&self) -> Result<Vec<Fiat>, Box<dyn std::error::Error>> {
        Ok(vec![
            Fiat::new("Singapore Dollar".to_string(), "SGD".to_string()),
            Fiat::new("Malaysian Ringgit".to_string(), "MYR".to_string()),
        ])
    }
}