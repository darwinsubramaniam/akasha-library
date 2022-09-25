use crate::crypto::model::Crypto;
use super::DatabaseDefaultRequirements;

#[async_trait::async_trait]
pub trait CryptoDBRequirement<'a>: DatabaseDefaultRequirements<Crypto<'a>> {
  async fn get_by_platform_defined_id(&self, platform_defined_id: &str) -> Result<Option<Crypto<'a>>, String>;  
}
