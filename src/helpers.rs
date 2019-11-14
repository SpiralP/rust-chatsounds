use async_std::{
  fs,
  fs::{File, OpenOptions},
  io::prelude::*,
};
use bytes::Bytes;
pub use rodio::{Decoder, Device, Sink, SpatialSink};
use sha2::{Digest, Sha256};
use std::path::Path;

pub async fn cache_download<S: AsRef<str>, P: AsRef<Path>>(url: S, cache_path: P) -> Bytes {
  let mut hasher = Sha256::new();
  hasher.input(url.as_ref());

  let hex = format!("{:x}", hasher.result());

  let hex_dir = &hex[0..2];
  let hex_filename = &hex[2..];

  let dir_path = cache_path.as_ref().join(hex_dir);
  if !fs::metadata(&dir_path)
    .await
    .map(|meta| meta.is_dir())
    .unwrap_or(false)
  {
    fs::create_dir(&dir_path).await.unwrap();
  }

  let file_path = dir_path.join(hex_filename);
  if !fs::metadata(&file_path)
    .await
    .map(|meta| meta.is_file())
    .unwrap_or(false)
  {
    let mut file = File::create(&file_path).await.unwrap();
    let bytes = reqwest::get(url.as_ref())
      .await
      .unwrap()
      .bytes()
      .await
      .unwrap();

    file.write_all(&bytes).await.unwrap();

    bytes
  } else {
    let mut file = OpenOptions::new()
      .read(true)
      .open(&file_path)
      .await
      .unwrap();

    let mut vec = Vec::new();
    file.read_to_end(&mut vec).await.unwrap();

    Bytes::from(vec)
  }
}
