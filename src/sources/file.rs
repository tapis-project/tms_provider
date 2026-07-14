use crate::errors::ProviderError;
use crate::sources::*;
use std::path::{Path, PathBuf};

pub struct FileSource {
    pub file: PathBuf,
}

impl Source for FileSource {
    fn get_provider(&self, provider_id: ProviderId) -> Result<Provider, ProviderError> {
        Ok(Provider {
            id: "provider foo".into(),
        })
    }

    fn get_account(
        &self,
        provider: &Provider,
        account_id: AccountId,
    ) -> Result<Account, ProviderError> {
        Ok(Account {
            id: "account foo".into(),
        })
    }

    fn get_resources(
        &self,
        provider: &Provider,
        account: &Account,
    ) -> Result<Resources, ProviderError> {
        Ok(Resources {
            provider_name: provider.id.clone(),
            resources: vec![],
        })
    }
}
