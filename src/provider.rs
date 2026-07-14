use crate::{errors::ProviderError, sources::{AccountId, ProviderId, Resources, Source}};

pub fn get_resources<S: Source>(
    data: &S,
    provider_id: ProviderId,
    account_id: AccountId,
) -> Result<Resources, ProviderError> 
{
    let provider = data.get_provider(provider_id)?;
    let account = data.get_account(&provider, account_id)?;
    data.get_resources(&provider, &account)
}