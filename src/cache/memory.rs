use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use bytes::Bytes;
use sha2::{Digest, Sha256};

pub async fn download<F>(
    url: &str,
    cache_path: &Path,
    use_etag: bool,
    validator: F,
) -> Result<Bytes>
where
    F: FnOnce(Bytes) -> Result<Result<Bytes>>,
{
    // let need_fetch = if use_etag {

    // };

    todo!();
}
