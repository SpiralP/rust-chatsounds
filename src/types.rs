use std::{path::Path, sync::Arc};

use anyhow::Result;
use async_trait::async_trait;
use bytes::Bytes;
pub use rodio::{queue::SourcesQueueOutput, Decoder, Device, Sample, Sink, Source, SpatialSink};

use crate::cache::download;

pub type BoxSource = Box<dyn Source<Item = i16> + Send>;

#[async_trait]
pub trait GetBytes {
    async fn get_bytes(&self, cache_path: &Path) -> Result<Bytes>;
}

#[derive(Clone)]
pub struct Chatsound {
    // Metastruct/garrysmod-chatsounds
    pub repo: String,
    // sound/chatsounds/autoadd
    pub repo_path: String,
    // 26e/nestetrismusic/1.ogg
    pub sound_path: String,
}

impl Chatsound {
    pub async fn load(&self, cache_path: &Path) -> Result<LoadedChatsound> {
        let url = format!(
            "https://raw.githubusercontent.com/{}/HEAD/{}/{}",
            self.repo, self.repo_path, self.sound_path
        );

        let bytes = download(&url, cache_path, false, |bytes| Ok(Ok(bytes))).await?;

        Ok(LoadedChatsound { bytes })
    }
}

pub struct LoadedChatsound {
    bytes: Bytes,
}

#[async_trait]
impl GetBytes for LoadedChatsound {
    async fn get_bytes(&self, _cache_path: &Path) -> Result<Bytes> {
        Ok(self.bytes.clone())
    }
}

#[async_trait]
impl GetBytes for Chatsound {
    async fn get_bytes(&self, cache_path: &Path) -> Result<Bytes> {
        let loaded_chatsound = self.load(cache_path).await?;
        loaded_chatsound.get_bytes(cache_path).await
    }
}

pub trait ChatsoundsSink {
    fn sleep_until_end(&mut self);
    fn stop(&mut self);
    fn set_volume(&mut self, volume: f32);
}

impl ChatsoundsSink for Arc<Sink> {
    fn sleep_until_end(&mut self) {
        Sink::sleep_until_end(self)
    }

    fn stop(&mut self) {
        Sink::stop(self)
    }

    fn set_volume(&mut self, volume: f32) {
        Sink::set_volume(self, volume)
    }
}

impl ChatsoundsSink for Arc<SpatialSink> {
    fn sleep_until_end(&mut self) {
        SpatialSink::sleep_until_end(self)
    }

    fn stop(&mut self) {
        SpatialSink::stop(self)
    }

    fn set_volume(&mut self, volume: f32) {
        SpatialSink::set_volume(self, volume)
    }
}
