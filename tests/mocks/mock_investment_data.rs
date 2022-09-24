use akasha::money_investment::{Investment, InvestmentType};
use chrono::{DateTime, NaiveDateTime, Utc};

#[allow(dead_code)]
pub(crate) struct MockInvestmentData {
    pub investments: Vec<Investment>,
}
#[allow(dead_code)]
impl MockInvestmentData {
    pub fn default() -> MockInvestmentData {
        let mut mock_currency: Vec<Investment> = Vec::new();
        mock_currency.push(Investment::new(
            Some("1".to_string()),
            InvestmentType::DEPOSIT,
            100.0,
            "SGD".to_string(),
            DateTime::from_utc(NaiveDateTime::from_timestamp(61, 0), Utc),
        ));
        mock_currency.push(Investment::new(
            Some("2".to_string()),
            InvestmentType::WITHDRAW,
            10.0,
            "SGD".to_string(),
            DateTime::from_utc(NaiveDateTime::from_timestamp(61, 0), Utc),
        ));
        mock_currency.push(Investment::new(
            Some("3".to_string()),
            InvestmentType::DEPOSIT,
            10.0,
            "MYR".to_string(),
            DateTime::from_utc(NaiveDateTime::from_timestamp(61, 0), Utc),
        ));
        MockInvestmentData {
            investments: mock_currency,
        }
    }
}