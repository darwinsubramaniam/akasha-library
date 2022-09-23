use std::{future, process::Output};

use crate::{
    database::investment::InvestmentDBServiceRequirement,
    fiat::{Fiat, FiatService},
};

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

    /// Sum of investment done particular currency
    /// ### Formula
    /// Total Deposit - Total Withdrawal
    pub async fn get_total_investment_by_currency(
        &self,
        currency: &str,
    ) -> Result<f64, Box<dyn std::error::Error>> {
        let investments = self.get_investments_by_currency(currency).await?;
        let deposit: f64 = investments
            .iter()
            .filter(|investment| investment.investment_type() == &InvestmentType::DEPOSIT)
            .map(|investment| investment.amount())
            .sum();

        let withdraw: f64 = investments
            .iter()
            .filter(|investment| investment.investment_type() == &InvestmentType::WITHDRAW)
            .map(|investment| investment.amount())
            .sum();
        Ok(deposit - withdraw)
    }

    /// Sum of investment in particular currency. Converted to desired [`Fiat`] currency
    /// ### Formula
    /// Total Deposit - Total Withdrawal
    pub async fn get_total_investment_by_type(
        &self,
        investment_type: &InvestmentType,
        desired_conversion: &Fiat,
        conversion_service: &dyn FiatService,
    ) -> Result<f64, Box<dyn std::error::Error>> {
        let investments = self.get_investments_by_type(investment_type).await?;
        let investments = investments
            .iter()
            .map(|investment| async move {
                let amount = investment.amount();
                let currency = investment.currency();
                let fiat = Fiat::new(currency.to_string(), currency.to_string());
                let conversion = fiat
                    .conversion(amount, desired_conversion, conversion_service)
                    .await
                    .unwrap();
                conversion
            })
            .collect::<Vec<_>>();

        let total = futures::future::join_all(investments).await.iter().sum();
        Ok(total)
    }

    /// Total left investment mixed of all currency but will be calculated based on desired currency
    /// ### Formula
    /// Total deposit - Total withdrawal
    pub async fn project_total_investment(
        &self,
        desired_conversion: &Fiat,
        conversion_service: &dyn FiatService,
    ) -> Result<f64, Box<dyn std::error::Error>> {
        let deposit = self
            .get_total_investment_by_type(
                &InvestmentType::DEPOSIT,
                desired_conversion,
                conversion_service,
            )
            .await?;
        let withdrawal = self
            .get_total_investment_by_type(
                &InvestmentType::WITHDRAW,
                desired_conversion,
                conversion_service,
            )
            .await?;
        Ok(deposit - withdrawal)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use crate::database::investment::InvestmentDBServiceRequirement;
    use crate::database::DatabaseServiceDefaultRequirements;
    use crate::fiat::FiatService;

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

       async  fn get(&self) -> Result<Vec<Investment>, Box<dyn std::error::Error>> {
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
}
