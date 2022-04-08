use anyhow::{Context, Result};
use bytes::Bytes;
use sha2::{Digest, Sha256};
use std::path::{Path, PathBuf};
use tokio::{
    fs,
    fs::{File, OpenOptions},
    io::{AsyncReadExt, AsyncWriteExt},
};

static APP_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));

async fn get<S: AsRef<str>>(url: S) -> Result<(Bytes, Option<String>)> {
    let client = reqwest::Client::builder()
        .user_agent(APP_USER_AGENT)
        .build()?;

    let response = client.get(url.as_ref()).send().await?;

    let etag = response
        .headers()
        .get(reqwest::header::ETAG)
        .and_then(|value| value.to_str().ok())
        .map(|value| value.to_string());

    let bytes = response.bytes().await?;

    Ok((bytes, etag))
}

async fn get_head_etag<S: AsRef<str>>(url: S) -> Result<Option<String>> {
    let client = reqwest::Client::builder()
        .user_agent(APP_USER_AGENT)
        .build()?;

    let response = client.head(url.as_ref()).send().await?;

    let etag = response
        .headers()
        .get(reqwest::header::ETAG)
        .and_then(|value| value.to_str().ok())
        .map(|value| value.to_string());

    Ok(etag)
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
        fs::create_dir(&dir_path).await?;
    }

    let file_path = dir_path.join(hex_filename);
    let etag_file_path = dir_path.join(format!("{}.etag", hex_filename));

    Ok((file_path, etag_file_path))
}

/// if `use_etag` is false, always used cached file if it exists
pub async fn cache_download<F>(
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
                .context("open etag_file_path")?;

            let mut s = String::new();
            etag_file.read_to_string(&mut s).await?;

            let maybe_etag = get_head_etag(&url).await.context("get_head_etag")?;

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
        read_file(&file_path)
            .await
            .with_context(|| format!("read_file {:?}", file_path))?
    } else {
        let (bytes, maybe_etag) = get(&url).await.with_context(|| format!("get {:?}", url))?;

        let maybe_bytes = validator(bytes)?;

        if let Ok(bytes) = maybe_bytes {
            let mut file = File::create(&file_path)
                .await
                .with_context(|| format!("File::create {:?}", file_path))?;
            file.write_all(&bytes).await?;

            if use_etag {
                if let Some(etag) = maybe_etag {
                    let mut file = File::create(&etag_file_path)
                        .await
                        .with_context(|| format!("File::create {:?}", etag_file_path))?;
                    file.write_all(etag.as_bytes()).await?;
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

async fn read_file(file_path: &Path) -> Result<Bytes> {
    let mut file = OpenOptions::new().read(true).open(&file_path).await?;

    let mut vec = Vec::new();
    file.read_to_end(&mut vec).await?;

    Ok(Bytes::from(vec))
}
