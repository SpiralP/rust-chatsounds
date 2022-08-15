use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

use bytes::Bytes;
use sha2::{Digest, Sha256};

use crate::{cache::utils::get, error::Result};

pub async fn download<F>(
    url: &str,
    fs_memory: &mut HashMap<String, Bytes>,
    use_etag: bool,
    validator: F,
) -> Result<Bytes>
where
    F: FnOnce(Bytes) -> Result<Result<Bytes>>,
{
    if let Some(bytes) = fs_memory.get(url) {
        Ok(bytes.clone())
    } else {
        let (bytes, maybe_etag) = get(url).await?;
        fs_memory.insert(url.to_owned(), bytes.clone());

        Ok(bytes)
    }
}
