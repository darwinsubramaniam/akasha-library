use self::{fiat::FiatDBServiceRequirement, investment::InvestmentDBServiceRequirement};

pub mod crypto;
pub mod fiat;
pub mod investment;

#[async_trait::async_trait]
pub trait DatabaseServiceDefaultRequirements<T> {
    /// Create a new entry in the database
    async fn create(&self, model: &T) -> Result<(), Box<dyn std::error::Error>>;
    /// Get all entries from the database
    async fn get(&self) -> Result<Vec<T>, Box<dyn std::error::Error>>;
    /// Get an entry from the database by id
    async fn get_by_id(&self, id: &str) -> Result<Option<T>, Box<dyn std::error::Error>>;
    /// Update an entry in the database by id
    async fn update_by_id(&self, id: &str, model: T) -> Result<(), Box<dyn std::error::Error>>;
    /// Delete an entry in the database by id
    async fn delete_by_id(&self, id: &str) -> Result<(), Box<dyn std::error::Error>>;
}

pub trait DatabaseRequirement: InvestmentDBServiceRequirement + FiatDBServiceRequirement {}
