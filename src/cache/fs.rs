use std::{
    path::{Path, PathBuf},
    time::Duration,
};

use bytes::Bytes;
use reqwest::header::HeaderValue;
use sha2::{Digest, Sha256};
use tokio::fs;

use super::utils::{get, get_with_etag};
use crate::{error::Result, Error};

// time we allow to use the cached file when we couldn't fetch anything
const ERROR_CACHE_FILE_DURATION: Duration = Duration::from_secs(24 * 60 * 60); // 1 day

/// when `use_cache` is true, always used cached file if it exists
pub async fn download(url: &str, cache_path: &Path, use_cache: bool) -> Result<Bytes> {
    let (file_path, etag_file_path) = cache_file_path(url, cache_path).await;

    if use_cache {
        if let Ok(bytes) = fs::read(&file_path).await {
            return Ok(Bytes::from(bytes));
        }
    }

    let file_exists = fs::try_exists(&file_path).await.unwrap_or(false);
    let maybe_etag = if file_exists {
        fs::read(&etag_file_path)
            .await
            .ok()
            .and_then(|vec| HeaderValue::from_bytes(&vec).ok())
    } else {
        None
    };

    let mut result = if let Some(etag) = maybe_etag {
        get_with_etag(url, etag).await
    } else {
        get(url).await.map(Some)
    };

    if result.is_err()
        && file_exists
        && fs::metadata(&file_path)
            .await
            .ok()
            .and_then(|meta| meta.modified().ok())
            .and_then(|modified| modified.elapsed().ok())
            .map(|elapsed| elapsed < ERROR_CACHE_FILE_DURATION)
            .unwrap_or(false)
    {
        // if we have a cache file, and it's younger than 1 day, and we couldn't fetch anything,
        // treat like the etag matched, and return the cached file
        result = Ok(None);
    }

    if let Some((bytes, maybe_etag)) = result? {
        fs::write(&file_path, &bytes)
            .await
            .map_err(|err| Error::Io {
                err,
                path: file_path,
            })?;
        if let Some(etag) = maybe_etag {
            fs::write(&etag_file_path, etag)
                .await
                .map_err(|err| Error::Io {
                    err,
                    path: etag_file_path,
                })?;
        }
        Ok(bytes)
    } else {
        let bytes = fs::read(&file_path).await.map_err(|err| Error::Io {
            err,
            path: file_path,
        })?;
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

#[cfg(test)]
#[tokio::test]
#[ignore]
async fn test_fs_download_use_cache() {
    let cache_path = PathBuf::from("cache");
    fs::create_dir_all(&cache_path).await.unwrap();

    let url = "https://httpbin.org/status/200";
    let (file_path, _etag_file_path) = cache_file_path(url, &cache_path).await;

    fs::remove_file(&file_path).await.ok();

    let bytes = download(url, &cache_path, true).await.unwrap();
    assert_ne!(bytes.as_ref(), b"asdf");

    fs::write(&file_path, b"asdf").await.unwrap();

    let bytes = download(url, &cache_path, true).await.unwrap();
    assert_eq!(bytes.as_ref(), b"asdf");
}
