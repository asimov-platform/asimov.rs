// This is free and unencumbered software released into the public domain.

use std::sync::Arc;

use rmcp::model::ResourceContents;

use super::server::Error;

pub type ResourceCallback = Arc<dyn Fn() -> Result<Vec<ResourceContents>, Error> + Send + Sync>;

#[derive(Clone)]
pub struct Resource {
    pub uri: String,
    pub name: String,
    pub description: Option<String>,
    pub mime_type: Option<String>,
    pub size: Option<u32>,
    pub callback: ResourceCallback,
}

impl Resource {
    pub fn new<U, N, D, M, F>(
        uri: U,
        name: N,
        description: Option<D>,
        mime_type: Option<M>,
        size: Option<u32>,
        callback: F,
    ) -> Self
    where
        U: Into<String>,
        N: Into<String>,
        D: Into<String>,
        M: Into<String>,
        F: Fn() -> Result<Vec<ResourceContents>, Error> + Send + Sync + 'static,
    {
        Self {
            uri: uri.into(),
            name: name.into(),
            description: description.map(Into::into),
            mime_type: mime_type.map(Into::into),
            size,
            callback: Arc::new(callback),
        }
    }
}

#[derive(Clone)]
pub struct ResourceTemplate {
    pub name: String,
    pub uri_template: String,
    pub description: Option<String>,
    pub mime_type: Option<String>,
}
