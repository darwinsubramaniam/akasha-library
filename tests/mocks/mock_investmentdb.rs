use super::mock_investment_data::MockInvestmentData;
use akasha::{
    database::{investment::InvestmentDBRequirement, DatabaseDefaultRequirements},
    money_investment::{Investment, InvestmentType},
};

/// user for as mock database, specifically for investment
/// all the values are hardcoded
pub struct MockInvestmentDBService {
    data: MockInvestmentData,
}

impl Default for MockInvestmentDBService {
    fn default() -> Self {
        MockInvestmentDBService {
            data: MockInvestmentData::default(),
        }
    }
}

#[async_trait::async_trait]
impl InvestmentDBRequirement for MockInvestmentDBService {
    async fn get_by_currency(
        &self,
        currency: &str,
    ) -> Result<Vec<&Investment>, Box<dyn std::error::Error>> {
        Ok(self
            .data
            .investments
            .iter()
            .filter(|investment| investment.currency() == currency)
            .collect::<Vec<&Investment>>())
    }

    async fn get_by_type(
        &self,
        investment_type: &InvestmentType,
    ) -> Result<Vec<&Investment>, Box<dyn std::error::Error>> {
        Ok(self
            .data
            .investments
            .iter()
            .filter(|investment| investment.investment_type() == investment_type)
            .collect::<Vec<&Investment>>())
    }
}

#[async_trait::async_trait]
impl DatabaseDefaultRequirements<Investment> for MockInvestmentDBService {
    async fn create(&mut self, investment: &Investment) -> Result<(), Box<dyn std::error::Error>> {
        Ok(self.data.investments.push(investment.clone()))
    }

    async fn get(&self) -> Result<&Vec<Investment>, Box<dyn std::error::Error>> {
        Ok(&self.data.investments)
    }

    async fn get_by_id(&self, id: &str) -> Result<Option<&Investment>, Box<dyn std::error::Error>> {
        Ok(self
            .data
            .investments
            .iter()
            .find(|investment| investment.id().unwrap() == id))
    }

    async fn update_by_id(
        &mut self,
        id: &str,
        investment: &Investment,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let index = self
            .data
            .investments
            .iter()
            .position(|investment| investment.id().unwrap() == id)
            .unwrap();
        Ok(self.data.investments[index] = investment.clone())
    }

    async fn delete_by_id(&mut self, id: &str) -> Result<(), Box<dyn std::error::Error>> {
        let index = self
            .data
            .investments
            .iter()
            .position(|investment| investment.id().unwrap() == id)
            .unwrap();
        self.data.investments.remove(index);
        Ok(())
    }
}
