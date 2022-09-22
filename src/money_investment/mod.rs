pub mod service;
use chrono::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub enum InvestmentType {
    DEPOSIT,
    WITHDRAW,
}

///Deposit entry
#[derive(Debug, Deserialize, Serialize)]
#[allow(dead_code)]
pub struct Investment {
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<String>,
    investment_type: InvestmentType,
    amount: f64,
    currency: String,
    date: DateTime<Utc>,
}

impl PartialEq for Investment {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
            && self.investment_type == other.investment_type
            && self.amount == other.amount
            && self.currency == other.currency
            && self.date == other.date
    }
}


impl<'a> Investment {
    pub fn new(
        investment_type: InvestmentType,
        amount: f64,
        currency: String,
        date: DateTime<Utc>,
    ) -> Self {
        Self {
            id: None,
            investment_type,
            amount,
            currency,
            date,
        }
    }

    pub fn id(&self) -> Option<&String> {
        self.id.as_ref()
    }

    pub fn investment_type(&self) -> &InvestmentType {
        &self.investment_type
    }

    pub fn amount(&self) -> &f64 {
        &self.amount
    }

    pub fn currency(&self) -> &String {
        &self.currency
    }

    pub fn invested_date(&self) -> &DateTime<Utc> {
        &&self.date
    }
}

#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn test_investment_new() {
        let investment = Investment::new(InvestmentType::DEPOSIT, 100.0, "USD".to_string(), Utc::now());
        assert_eq!(investment.investment_type(), &InvestmentType::DEPOSIT);
        assert_eq!(investment.amount(), &100.0);
        assert_eq!(investment.currency(), &"USD".to_string());
    }
}