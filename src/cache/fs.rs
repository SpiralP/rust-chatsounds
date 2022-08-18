use std::path::{Path, PathBuf};

use bytes::Bytes;
use quick_error::ResultExt;
use reqwest::header::HeaderValue;
use sha2::{Digest, Sha256};
use tokio::fs;

use super::utils::{get, get_with_etag};
use crate::error::Result;

/// if `use_etag` is false, always used cached file if it exists
pub async fn download(url: &str, cache_path: &Path, use_etag: bool) -> Result<Bytes> {
    let (file_path, etag_file_path) = cache_file_path(url, cache_path).await;

    if !use_etag {
        // use cached file (if it exists)
        let maybe_bytes = fs::read(&etag_file_path).await.ok();
        if let Some(bytes) = maybe_bytes {
            return Ok(Bytes::from(bytes));
        }
    }

    let maybe_etag = if use_etag {
        fs::read(&etag_file_path)
            .await
            .ok()
            .and_then(|vec| HeaderValue::from_bytes(&vec).ok())
    } else {
        None
    };

    let maybe_response = if let Some(etag) = maybe_etag {
        get_with_etag(url, etag).await?
    } else {
        Some(get(url).await?)
    };

    if let Some((bytes, maybe_etag)) = maybe_response {
        fs::write(&file_path, &bytes).await.context(&file_path)?;
        if use_etag {
            if let Some(etag) = maybe_etag {
                fs::write(&etag_file_path, etag)
                    .await
                    .context(&etag_file_path)?;
            }
        }
        Ok(bytes)
    } else {
        let bytes = fs::read(&file_path).await.context(&file_path)?;
        Ok(Bytes::from(bytes))
    }
}

async fn cache_file_path(url: &str, cache_path: &Path) -> (PathBuf, PathBuf) {
    let hex = {
        let mut hasher = Sha256::new();
        hasher.update(url);
        format!("{:x}", hasher.finalize())
    };

    let hex_dir = &hex[0..2];
    let hex_filename = &hex[2..];

    let dir_path = cache_path.join(hex_dir);
    fs::create_dir(&dir_path).await.ok();

    let file_path = dir_path.join(hex_filename);
    let etag_file_path = dir_path.join(format!("{}.etag", hex_filename));

    (file_path, etag_file_path)
}
