use serde::Serialize;
use uuid::Uuid;

use crate::errors::ProviderError;

pub mod file;

pub type ProviderId = String;
pub type AccountId = String;

pub struct Provider {
    id: String,
}

pub struct Account {
    id: String,
}

#[derive(Serialize)]
pub struct Resources {
    pub provider_name: String,
    pub resources: Vec<Resource>,
}

#[derive(Serialize)]
pub struct Resource {
    pub id: Uuid,
    pub name: String,
    pub description: String,
}

pub trait Source {
    fn get_provider(&self, provider_id: ProviderId) -> Result<Provider, ProviderError>;
    fn get_account(
        &self,
        provider: &Provider,
        account_id: AccountId,
    ) -> Result<Account, ProviderError>;
    fn get_resources(
        &self,
        provider: &Provider,
        account: &Account,
    ) -> Result<Resources, ProviderError>;
}
