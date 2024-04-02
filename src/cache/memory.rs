use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

use bytes::Bytes;
use sha2::{Digest, Sha256};

use crate::{cache::utils::get, error::Result};

pub async fn download(url: &str, fs_memory: crate::FsMemory, _use_cache: bool) -> Result<Bytes> {
    {
        let fs_memory = fs_memory.read().await;
        if let Some(bytes) = fs_memory.get(url) {
            return Ok(bytes.clone());
        }
    }

    let (bytes, maybe_etag) = get(url).await?;
    {
        let mut fs_memory = fs_memory.write().await;
        fs_memory.insert(url.to_owned(), bytes.clone());
    }

    Ok(bytes)
}
