use super::DatabaseDefaultRequirements;
use crate::crypto::model::Crypto;

#[async_trait::async_trait]
pub trait CryptoDBRequirement: DatabaseDefaultRequirements<Crypto> {
    async fn get_by_platform_defined_id(
        &self,
        platform_defined_id: &str,
    ) -> Result<Option<Crypto>, Box<dyn std::error::Error>>;
}
