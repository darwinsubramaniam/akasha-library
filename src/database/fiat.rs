use crate::fiat::Fiat;
use super::DatabaseServiceDefaultRequirements;

#[async_trait::async_trait]
pub trait FiatDBServiceRequirement: DatabaseServiceDefaultRequirements<Fiat> {}
