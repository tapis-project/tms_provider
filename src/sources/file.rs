use ordermap::OrderSet;
use serde::Deserialize;

use crate::{
    errors::{
        ProviderError,
        SourceError::{AccountNotFound, ResourceNotFound},
    },
    sources::*,
};
use std::{
    collections::HashMap,
    fs::read_to_string,
    path::{Path, PathBuf},
};

pub struct FileSource {
    _file: PathBuf,
    data: ResourcesData,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
struct ResourcesData {
    resources: HashMap<ResourceId, Resource>,
    generally_available: Vec<ResourceId>,
    per_account: HashMap<AccountId, Vec<ResourceId>>,
}

#[async_trait]
impl Source for FileSource {
    async fn get_resources(
        &self,
        account: Option<AccountId>,
    ) -> Result<Resources, SourceError> {
        let resources = &self.data.resources;
        let empty = vec![];
        let per_account = account
            .map(|act| {
                self.data
                    .per_account
                    .get(&act)
                    .ok_or_else(|| AccountNotFound(act))
            })
            .transpose()?
            .unwrap_or(&empty);
        Ok(Resources {
            resources: self
                .data
                .generally_available
                .iter()
                .chain(per_account)
                .map(|r| {
                    resources
                        .get(r)
                        .cloned()
                        .ok_or_else(|| ResourceNotFound(r.into()))
                })
                .collect::<Result<OrderSet<_>, _>>()?
                .into_iter()
                .collect::<Vec<_>>(),
        })
    }
}

impl FileSource {
    pub fn from_path(file: &Path) -> Result<Self, ProviderError> {
        let s = read_to_string(file)?;
        let src = FileSource {
            _file: file.into(),
            data: serde_yaml::from_str(&s)
                .map_err(|e| ProviderError::ApplicationConfigError(e.to_string()))?,
        };
        Ok(src)
    }
}
