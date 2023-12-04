use anyhow::Context;
use std::{marker::PhantomData, path::PathBuf};

use super::Error;

#[derive(Clone)]
pub struct StoredJSON<T> {
    path: PathBuf,
    // last_contents: tokio::sync::Mutex<Option<String>>,
    _marker: PhantomData<T>,
}

impl<T> std::fmt::Debug for StoredJSON<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("StoredJSON")
            .field("path", &self.path)
            .finish()
    }
}

/// Helper so we can use [tracing::instrument] without implementing [std::fmt::Debug] for inner type.
pub struct Opaque<T>(pub T);

impl<T> std::fmt::Debug for Opaque<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Opaque").finish()
    }
}

impl<T> StoredJSON<T> {
    pub fn new(path: PathBuf) -> Self {
        StoredJSON {
            path,
            // last_contents: Default::default(),
            _marker: PhantomData,
        }
    }
}

impl<T: serde::Serialize + serde::de::DeserializeOwned> StoredJSON<T> {
    #[tracing::instrument]
    pub async fn read(&self) -> Result<Option<Opaque<T>>, Error> {
        if !tokio::fs::try_exists(&self.path)
            .await
            .with_context(|| format!("Checking if file exists at {:?}", self.path))?
        {
            return Ok(None);
        }
        let contents = tokio::fs::read(&self.path)
            .await
            .with_context(|| format!("Reading file from {:?}", self.path))?;
        let value = serde_json::from_slice(&contents)
            .with_context(|| format!("Parsing json file from {:?}", self.path))?;
        tracing::info!(path=?self.path, "read stored file");
        Ok(Some(Opaque(value)))
    }
    #[tracing::instrument(skip(value))]
    pub async fn write(&self, value: &T) -> Result<(), Error> {
        let value = serde_json::to_vec_pretty(&value).context("To json")?;
        tokio::fs::write(&self.path, &value)
            .await
            .with_context(|| format!("Writing JSON file to {:?}", self.path))?;
        tracing::info!(path=?self.path, "wrote file");
        Ok(())
    }
}
