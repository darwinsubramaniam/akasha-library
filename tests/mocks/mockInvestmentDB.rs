use akasha::{
    database::{investment::InvestmentDBServiceRequirement, DatabaseServiceDefaultRequirements},
    money_investment::{Investment, InvestmentType},
};

pub struct MockInvestmentDBService {}

#[async_trait::async_trait]
impl InvestmentDBServiceRequirement for MockInvestmentDBService {
    async fn get_by_currency(
        &self,
        currency: &str,
    ) -> Result<Vec<Investment>, Box<dyn std::error::Error>> {
        todo!()
    }

    async fn get_by_type(
        &self,
        investment_type: &InvestmentType,
    ) -> Result<Vec<Investment>, Box<dyn std::error::Error>> {
        todo!()
    }
}

#[async_trait::async_trait]
impl DatabaseServiceDefaultRequirements<Investment> for MockInvestmentDBService {
    async fn create(&self, investment: &Investment) -> Result<(), Box<dyn std::error::Error>> {
        todo!()
    }

    async fn get(&self) -> Result<Vec<Investment>, Box<dyn std::error::Error>> {
        todo!()
    }

    async fn get_by_id(&self, id: &str) -> Result<Option<Investment>, Box<dyn std::error::Error>> {
        todo!()
    }

    async fn update_by_id(
        &self,
        id: &str,
        investment: Investment,
    ) -> Result<(), Box<dyn std::error::Error>> {
        todo!()
    }

    async fn delete_by_id(&self, id: &str) -> Result<(), Box<dyn std::error::Error>> {
        todo!()
    }
}
