use std::{path::Path, sync::Arc};

use bytes::Bytes;
pub use rodio::{queue::SourcesQueueOutput, Decoder, Device, Sample, Sink, Source, SpatialSink};

use crate::{cache::download, error::Result};

pub type BoxSource = Box<dyn Source<Item = i16> + Send>;

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
    pub fn get_url(&self) -> String {
        format!(
            "https://raw.githubusercontent.com/{}/HEAD/{}/{}",
            self.repo, self.repo_path, self.sound_path
        )
    }

    #[cfg(feature = "fs")]
    pub async fn load(&self, cache_path: &Path) -> Result<Bytes> {
        let url = self.get_url();
        download(&url, cache_path, false, |bytes| Ok(Ok(bytes))).await
    }

    #[cfg(not(feature = "fs"))]
    pub async fn load(
        &self,
        fs_memory: &mut std::collections::HashMap<String, Bytes>,
    ) -> Result<Bytes> {
        let url = self.get_url();
        download(&url, fs_memory, false, |bytes| Ok(Ok(bytes))).await
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
