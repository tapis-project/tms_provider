use std::{
    collections::HashMap,
    fs::read_to_string,
    path::{Path, PathBuf},
};

use crate::{
    errors::ProviderError,
    types::{AccountId, ResourceId},
};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use url::Url;

use crate::errors::SourceError;

pub mod file;
pub mod null;

#[derive(Serialize, Deserialize, Default)]
pub struct Resources {
    pub resources: Vec<Resource>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Resource {
    pub resource_id: ResourceId,
    pub name: String,
    pub url: Url,
    pub description: String,
}

#[async_trait]
pub trait Source {
    async fn get_resources(
        &self,
        account: Option<AccountId>,
    ) -> Result<Resources, SourceError>;
}
