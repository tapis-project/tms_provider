use async_trait::async_trait;

use super::{AccountId, Resources};
use crate::errors::SourceError;
use crate::sources::Source;

pub struct NullSource;

#[async_trait]
impl Source for NullSource {
    async fn get_resources(
        &self,
        _account: Option<AccountId>,
    ) -> Result<Resources, SourceError> {
        Ok(Resources { resources: vec![] })
    }
}
