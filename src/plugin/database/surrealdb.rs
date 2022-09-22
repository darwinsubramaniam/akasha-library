use crate::{
    money_investment::{Investment, InvestmentType}, fiat::Fiat, database::{investment::InvestmentDBServiceRequirement, fiat::FiatDBServiceRequirement, DatabaseRequirement, DatabaseServiceDefaultRequirements},
};
use reqwest::Url;

#[derive(Debug)]
#[allow(dead_code)]
pub struct SurrealDB {
    url: Url,
    namespace: String,
    database_name: String,
    username: String,
    password: String,
    client: reqwest::Client,
}

impl SurrealDB {
    pub fn new(
        url: String,
        namespace: String,
        database_name: String,
        username: String,
        password: String,
    ) -> Self {
        SurrealDB {
            url: Url::parse(&url).unwrap(),
            namespace,
            database_name,
            username,
            password,
            client: reqwest::Client::new(),
        }
    }
}

impl ToString for Investment {
    /// Convert the investment surrealdb format
    fn to_string(&self) -> String {
        if self.id().is_some() {
            format!("id = '{}' investment_type = '{:?}' amount = {} currency = {} date = {}", self.id().as_ref().unwrap(), self.investment_type() , self.amount(), self.currency(), self.invested_date())
        } else {
            format!("investment_type = '{:?}' amount = {} currency = {} date = {}", self.investment_type() , self.amount(), self.currency(), self.invested_date())
        }
    }
}

#[cfg(test)]
mod investment_test{
    use super::*;
    use chrono::Utc;
    #[test]
    fn test_to_string(){
        let investment = Investment::new(InvestmentType::DEPOSIT, 100.0, "USD".to_string(), Utc::now());
        println!("{}", investment.to_string());
        let expect_result = format!("investment_type = '{:?}' amount = {} currency = {} date = {}", investment.investment_type() , investment.amount(), investment.currency(), investment.invested_date());
        assert_eq!(investment.to_string(), expect_result);
    }
}

#[async_trait::async_trait]
impl DatabaseServiceDefaultRequirements<Investment> for SurrealDB{
        /// Create a new entry in the database
        async fn create(&self, model: &Investment) -> Result<(), Box<dyn std::error::Error>>{
            unimplemented!()
        }
        /// Get all entries from the database
        async fn get(&self) -> Result<Vec<Investment>, Box<dyn std::error::Error>>{
            unimplemented!()
        }
        /// Get an entry from the database by id
        async fn get_by_id(&self, id: &str) -> Result<Option<Investment>, Box<dyn std::error::Error>>{
            unimplemented!()
        }
        /// Update an entry in the database by id
        async fn update_by_id(&self, id: &str, model: Investment) -> Result<(), Box<dyn std::error::Error>>{
            unimplemented!()
        }
        /// Delete an entry in the database by id
        async fn delete_by_id(&self, id: &str) -> Result<(), Box<dyn std::error::Error>>{
            unimplemented!()
        }
}

#[async_trait::async_trait]
impl InvestmentDBServiceRequirement for SurrealDB { 
    async fn get_by_currency(&self, currency: &str) -> Result<Vec<Investment>, Box<dyn std::error::Error>>{
        unimplemented!()
    }
    async fn get_by_type(
        &self,
        investment_type: &InvestmentType,
    ) -> Result<Vec<Investment>, Box<dyn std::error::Error>>{
        unimplemented!()
    }
}

#[async_trait::async_trait]
impl DatabaseServiceDefaultRequirements<Fiat> for SurrealDB {
    /// Create a new entry in the database
    async fn create(&self, model: &Fiat) -> Result<(), Box<dyn std::error::Error>>{
        unimplemented!()
    }
    /// Get all entries from the database
    async fn get(&self) -> Result<Vec<Fiat>, Box<dyn std::error::Error>>{
        unimplemented!()
    }
    /// Get an entry from the database by id
    async fn get_by_id(&self, id: &str) -> Result<Option<Fiat>, Box<dyn std::error::Error>>{
        unimplemented!()
    }
    /// Update an entry in the database by id
    async fn update_by_id(&self, id: &str, model: Fiat) -> Result<(), Box<dyn std::error::Error>>{
        unimplemented!()
    }
    /// Delete an entry in the database by id
    async fn delete_by_id(&self, id: &str) -> Result<(), Box<dyn std::error::Error>>{
        unimplemented!()
    } 
}

#[async_trait::async_trait]
impl FiatDBServiceRequirement for SurrealDB {
}

impl DatabaseRequirement for SurrealDB {}
