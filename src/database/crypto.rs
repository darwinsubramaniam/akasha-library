use crate::crypto::model::Crypto;
use super::DatabaseServiceDefaultRequirements;

#[async_trait::async_trait]
pub trait CryptoDBServiceRequirement<'a>: DatabaseServiceDefaultRequirements<Crypto<'a>> {
  async fn get_by_platform_defined_id(&self, platform_defined_id: &str) -> Result<Option<Crypto<'a>>, String>;  
}
