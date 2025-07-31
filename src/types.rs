use std::{path::Path, sync::Arc};

use bytes::Bytes;
use rodio::{Sample, Sink, Source, SpatialSink};

#[cfg(feature = "playback")]
use crate::channel_volume::ChannelVolumeSink;
use crate::{cache::download, error::Result};

pub type BoxSource = Box<dyn Source<Item = Sample> + Send>;

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
    #[must_use]
    pub fn get_url(&self) -> String {
        format!(
            "https://raw.githubusercontent.com/{}/HEAD/{}/{}",
            self.repo, self.repo_path, self.sound_path
        )
    }

    #[cfg(feature = "fs")]
    pub async fn load(&self, cache_path: &Path) -> Result<Bytes> {
        let url = self.get_url();
        download(&url, cache_path, true).await
    }

    #[cfg(feature = "memory")]
    pub async fn load(&self, fs_memory: crate::FsMemory) -> Result<Bytes> {
        let url = self.get_url();
        download(&url, fs_memory, true).await
    }
}

#[cfg(feature = "playback")]
pub trait ChatsoundsSink {
    fn sleep_until_end(&self);
    fn stop(&self);
    fn set_volume(&self, volume: f32);
}

#[cfg(feature = "playback")]
impl ChatsoundsSink for Arc<Sink> {
    fn sleep_until_end(&self) {
        Sink::sleep_until_end(self);
    }

    fn stop(&self) {
        Sink::stop(self);
    }

    fn set_volume(&self, volume: f32) {
        Sink::set_volume(self, volume);
    }
}

#[cfg(feature = "playback")]
impl ChatsoundsSink for Arc<SpatialSink> {
    fn sleep_until_end(&self) {
        SpatialSink::sleep_until_end(self);
    }

    fn stop(&self) {
        SpatialSink::stop(self);
    }

    fn set_volume(&self, volume: f32) {
        SpatialSink::set_volume(self, volume);
    }
}

#[cfg(feature = "playback")]
impl ChatsoundsSink for Arc<ChannelVolumeSink> {
    fn sleep_until_end(&self) {
        Sink::sleep_until_end(&self.sink);
    }

    fn stop(&self) {
        Sink::stop(&self.sink);
    }

    fn set_volume(&self, volume: f32) {
        Sink::set_volume(&self.sink, volume);
    }
}
