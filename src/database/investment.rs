use crate::money_investment::{Investment, InvestmentType};
use super::DatabaseServiceDefaultRequirements;

/// This is trait which has to be be implemented by a service which sits between database and the investment.
#[async_trait::async_trait]
pub trait InvestmentDBServiceRequirement: DatabaseServiceDefaultRequirements<Investment> {
    async fn get_by_currency(
        &self,
        currency: &str,
    ) -> Result<Vec<Investment>, Box<dyn std::error::Error>>;
    async fn get_by_type(
        &self,
        investment_type: &InvestmentType,
    ) -> Result<Vec<Investment>, Box<dyn std::error::Error>>;
}
