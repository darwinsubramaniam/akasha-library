use crate::money_investment::{Investment, InvestmentType};
use super::DatabaseServiceDefaultRequirements;

/// This is trait which has to be be implemented by a service which sits between database and the investment.
/// the service should be able to get all the investment by currency and investment type
#[async_trait::async_trait]
pub trait InvestmentDBServiceRequirement: DatabaseServiceDefaultRequirements<Investment> {
    /// The investment filtered by currency
    async fn get_by_currency(
        &self,
        currency: &str,
    ) -> Result<Vec<&Investment>, Box<dyn std::error::Error>>;
    /// The investment filtered by investment type
    async fn get_by_type(
        &self,
        investment_type: &InvestmentType,
    ) -> Result<Vec<&Investment>, Box<dyn std::error::Error>>;
}
