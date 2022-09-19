use chrono::prelude::*;

#[derive(Debug)]
pub enum InvestmentType {
    DEPOSIT,
    WITHDRAW,
}

///Deposit entry
#[derive(Debug)]
pub struct Investment {
    pub investment_type: InvestmentType,
    pub amount: f64,
    pub currency: String,
    pub date: DateTime<Utc>,
}
