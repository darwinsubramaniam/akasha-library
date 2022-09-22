use crate::database::investment::InvestmentDBServiceRequirement;

use super::{Investment, InvestmentType};

pub struct InvestmentService<'a> {
    db: &'a dyn InvestmentDBServiceRequirement,
}

impl<'a> InvestmentService<'a> {
    pub fn new(db: &'a dyn InvestmentDBServiceRequirement) -> Self {
        InvestmentService { db }
    }

    pub async fn add_investment(
        &self,
        investment: &Investment,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let result = self.db.create(investment).await?;
        Ok(result)
    }

    pub async fn get_investments(&self) -> Result<Vec<Investment>, Box<dyn std::error::Error>> {
        Ok(self.db.get().await?)
    }

    pub async fn get_investments_by_currency(
        &self,
        currency: &str,
    ) -> Result<Vec<Investment>, Box<dyn std::error::Error>> {
        Ok(self.db.get_by_currency(currency).await?)
    }

    pub async fn get_investments_by_type(
        &self,
        investment_type: &InvestmentType,
    ) -> Result<Vec<Investment>, Box<dyn std::error::Error>> {
        Ok(self.db.get_by_type(investment_type).await?)
    }

    pub async fn get_investment_by_id(
        &self,
        id: &str,
    ) -> Result<Option<Investment>, Box<dyn std::error::Error>> {
        Ok(self.db.get_by_id(id).await?)
    }

    pub async fn update_investment_by_id(
        &self,
        id: &str,
        investment: Investment,
    ) -> Result<(), Box<dyn std::error::Error>> {
        Ok(self.db.update_by_id(id, investment).await?)
    }

    pub async fn delete_investment_by_id(
        &self,
        id: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        Ok(self.db.delete_by_id(id).await?)
    }
}
