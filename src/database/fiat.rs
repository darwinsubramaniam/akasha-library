use crate::fiat::Fiat;
use super::DatabaseDefaultRequirements;

#[async_trait::async_trait]
pub trait FiatDBRequirement: DatabaseDefaultRequirements<Fiat> {}
