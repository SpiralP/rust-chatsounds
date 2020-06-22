use crate::error::*;
use async_std::{
    fs,
    fs::{File, OpenOptions},
    io::prelude::*,
};
use bytes::Bytes;
use sha2::{Digest, Sha256};
use std::path::Path;

// Name your user agent after your app?
static APP_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));

pub async fn cache_download<S: AsRef<str>, P: AsRef<Path>>(url: S, cache_path: P) -> Result<Bytes> {
    let mut hasher = Sha256::new();
    hasher.update(url.as_ref());

    let hex = format!("{:x}", hasher.finalize());

    let hex_dir = &hex[0..2];
    let hex_filename = &hex[2..];

    let dir_path = cache_path.as_ref().join(hex_dir);
    if !fs::metadata(&dir_path)
        .await
        .map(|meta| meta.is_dir())
        .unwrap_or(false)
    {
        fs::create_dir(&dir_path).await?;
    }

    let file_path = dir_path.join(hex_filename);
    let bytes = if fs::metadata(&file_path)
        .await
        .map(|meta| meta.is_file() && meta.len() != 0)
        .unwrap_or(false)
    {
        let mut file = OpenOptions::new().read(true).open(&file_path).await?;

        let mut vec = Vec::new();
        file.read_to_end(&mut vec).await?;

        Bytes::from(vec)
    } else {
        let client = reqwest::Client::builder()
            .user_agent(APP_USER_AGENT)
            .build()?;

        let mut file = File::create(&file_path).await?;
        let bytes = client.get(url.as_ref()).send().await?.bytes().await?;

        file.write_all(&bytes).await?;

        bytes
    };

    Ok(bytes)
}
