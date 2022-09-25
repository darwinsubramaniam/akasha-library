use crate::{
    database::investment::InvestmentDBRequirement,
    fiat::{Fiat, FiatService},
};

use super::InvestmentType;


pub struct InvestmentService<'a> {
    db: &'a mut dyn InvestmentDBRequirement,
}

impl<'a> InvestmentService<'a> {
    pub fn new(db: &'a mut dyn InvestmentDBRequirement) -> Self {
        InvestmentService { db }
    }

    pub fn read_db(&self) -> &dyn InvestmentDBRequirement {
        self.db
    }
    pub fn write_db(&mut self) -> &mut dyn InvestmentDBRequirement {
        self.db
    }
}

impl<'a> InvestmentService<'a> {
    /// Sum of investment done particular currency
    /// ### Formula
    /// Total Deposit - Total Withdrawal
    pub async fn total_by_currency(
        &self,
        currency: &str,
    ) -> Result<f64, Box<dyn std::error::Error>> {
        let investments = self.read_db().get_by_currency(currency).await?;
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

    /// Sum of investment in particular currency.
    /// Converted to desired [`Fiat`] currency
    /// ### Formula
    /// Total Deposit - Total Withdrawal
    pub async fn total_by_type(
        &self,
        investment_type: &InvestmentType,
        desired_conversion: &Fiat,
        conversion_service: &dyn FiatService,
    ) -> Result<f64, Box<dyn std::error::Error>> {
        let investments = self.read_db().get_by_type(investment_type).await?;
        let investments = investments
            .iter()
            .map(|investment| async move {
                let amount = investment.amount();
                let currency = investment.currency();
                let fiat = Fiat::new(currency.to_string(), currency.to_string());
                if currency == desired_conversion.symbol() {
                    println!(
                        "Same Convertion {} {} to {}",
                        amount,
                        currency,
                        desired_conversion.symbol()
                    );
                    amount.to_owned()
                } else {
                    let conversion = fiat
                        .conversion(amount, desired_conversion, conversion_service)
                        .await
                        .unwrap();
                    println!(
                        "Fetch Convertion {} {} to {} = {} {}",
                        amount,
                        currency,
                        desired_conversion.symbol(),
                        conversion,
                        desired_conversion.symbol()
                    );
                    conversion
                }
            })
            .collect::<Vec<_>>();

        let total = futures::future::join_all(investments).await.iter().sum();
        Ok(total)
    }

    /// Total left investment mixed of all currency
    /// but will be calculated based on desired currency
    /// ### Formula
    /// Total deposit - Total withdrawal
    pub async fn total(
        &self,
        desired_conversion: &Fiat,
        conversion_service: &dyn FiatService,
    ) -> Result<f64, Box<dyn std::error::Error>> {
        let deposit = self
            .total_by_type(
                &InvestmentType::DEPOSIT,
                desired_conversion,
                conversion_service,
            )
            .await?;
        let withdrawal = self
            .total_by_type(
                &InvestmentType::WITHDRAW,
                desired_conversion,
                conversion_service,
            )
            .await?;
        Ok(deposit - withdrawal)
    }
}
