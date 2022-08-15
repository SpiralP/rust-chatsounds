use std::path::{Path, PathBuf};

use bytes::Bytes;
use quick_error::ResultExt;
use sha2::{Digest, Sha256};
#[cfg(feature = "fs")]
use tokio::{
    fs,
    fs::{File, OpenOptions},
    io::{AsyncReadExt, AsyncWriteExt},
};

use super::utils::{get, head_etag};
use crate::error::Result;

/// if `use_etag` is false, always used cached file if it exists
pub async fn download<F>(
    url: &str,
    cache_path: &Path,
    use_etag: bool,
    validator: F,
) -> Result<Bytes>
where
    F: FnOnce(Bytes) -> Result<Result<Bytes>>,
{
    let (file_path, etag_file_path) = cache_file_path(url, cache_path).await?;

    let file_cached = fs::metadata(&file_path)
        .await
        .map(|meta| meta.is_file() && meta.len() != 0)
        .unwrap_or(false);

    let etag_matches = if use_etag {
        let etag_file_cached = fs::metadata(&etag_file_path)
            .await
            .map(|meta| meta.is_file() && meta.len() != 0)
            .unwrap_or(false);

        // if we have everything cached
        if file_cached && etag_file_cached {
            let mut etag_file = OpenOptions::new()
                .read(true)
                .open(&etag_file_path)
                .await
                .context(&etag_file_path)?;

            let mut s = String::new();
            etag_file
                .read_to_string(&mut s)
                .await
                .context(&etag_file_path)?;

            let maybe_etag = head_etag(url).await?;

            if let Some(etag) = maybe_etag {
                // check if old matches current
                s == etag
            } else {
                // didn't have header, can't use cache
                false
            }
        } else {
            // didn't have either etag file cached or data file cached
            false
        }
    } else {
        // pretend it matches
        true
    };

    let use_cached = file_cached && etag_matches;

    Ok(if use_cached {
        read_file(&file_path).await?
    } else {
        let (bytes, maybe_etag) = get(url).await?;

        let maybe_bytes = validator(bytes)?;

        if let Ok(bytes) = maybe_bytes {
            let mut file = File::create(&file_path).await.context(&file_path)?;
            file.write_all(&bytes).await.context(&file_path)?;

            if use_etag {
                if let Some(etag) = maybe_etag {
                    let mut file = File::create(&etag_file_path)
                        .await
                        .context(&etag_file_path)?;
                    file.write_all(etag.as_bytes())
                        .await
                        .context(&etag_file_path)?;
                }
            }

            bytes
        } else {
            // light error, try using cache if it exists
            if let Ok(cached_bytes) = read_file(&file_path).await {
                cached_bytes
            } else {
                maybe_bytes?
            }
        }
    })
}

async fn cache_file_path(url: &str, cache_path: &Path) -> Result<(PathBuf, PathBuf)> {
    let mut hasher = Sha256::new();
    hasher.update(url);
    let hex = format!("{:x}", hasher.finalize());

    let hex_dir = &hex[0..2];
    let hex_filename = &hex[2..];

    let dir_path = cache_path.join(hex_dir);

    if !fs::metadata(&dir_path)
        .await
        .map(|meta| meta.is_dir())
        .unwrap_or(false)
    {
        fs::create_dir(&dir_path).await.context(&dir_path)?;
    }

    let file_path = dir_path.join(hex_filename);
    let etag_file_path = dir_path.join(format!("{}.etag", hex_filename));

    Ok((file_path, etag_file_path))
}

async fn read_file(file_path: &Path) -> Result<Bytes> {
    let mut file = OpenOptions::new()
        .read(true)
        .open(&file_path)
        .await
        .context(file_path)?;

    let mut vec = Vec::new();
    file.read_to_end(&mut vec).await.context(file_path)?;

    Ok(Bytes::from(vec))
}
