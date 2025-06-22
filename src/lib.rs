#![warn(clippy::nursery, clippy::pedantic)]
#![allow(clippy::missing_errors_doc, clippy::missing_panics_doc)]

mod cache;
mod channel_volume;
mod error;
mod fetching;
mod parsing;
mod types;

use std::{
    collections::{HashMap, VecDeque},
    io::{BufReader, Cursor},
    path::{Path, PathBuf},
    sync::Arc,
};

pub use bytes::Bytes;
use rand::prelude::*;
pub use rodio;
use rodio::{Decoder, Sink, SpatialSink};
#[cfg(feature = "playback")]
use rodio::{OutputStream, OutputStreamHandle};

pub use self::{
    channel_volume::ChannelVolumeSink,
    error::Error,
    fetching::{GitHubApiFileEntry, GitHubApiTrees, GitHubMsgpackEntries},
    types::Chatsound,
};
use self::{
    error::Result,
    parsing::{parse, ModifierTrait},
    types::{BoxSource, ChatsoundsSink},
};

#[cfg(feature = "memory")]
type FsMemory = Arc<async_lock::RwLock<HashMap<String, Bytes>>>;

pub struct Chatsounds {
    #[cfg(feature = "fs")]
    cache_path: PathBuf,
    // [sentence]: Chatsound[]
    map_store: HashMap<String, Vec<Chatsound>>,

    #[cfg(feature = "playback")]
    max_sinks: usize,
    #[cfg(feature = "playback")]
    volume: f32,

    #[cfg(feature = "playback")]
    _output_stream: OutputStream,
    #[cfg(feature = "playback")]
    output_stream_handle: OutputStreamHandle,
    #[cfg(feature = "playback")]
    sinks: VecDeque<Box<dyn ChatsoundsSink>>,

    #[cfg(feature = "memory")]
    fs_memory: FsMemory,
}

// TODO ???
#[allow(clippy::non_send_fields_in_send_ty)]
unsafe impl Send for Chatsounds {}
unsafe impl Sync for Chatsounds {}

impl Chatsounds {
    pub fn new(#[cfg(feature = "fs")] cache_path: &Path) -> Result<Self> {
        #[cfg(feature = "fs")]
        let cache_path = cache_path.canonicalize().map_err(|err| Error::Io {
            err,
            path: cache_path.into(),
        })?;
        #[cfg(feature = "fs")]
        ensure!(cache_path.is_dir(), Error::DirMissing { path: cache_path });

        #[cfg(feature = "playback")]
        let (output_stream, output_stream_handle) = OutputStream::try_default()?;

        Ok(Self {
            #[cfg(feature = "fs")]
            cache_path,
            map_store: HashMap::new(),

            #[cfg(feature = "playback")]
            max_sinks: 16,
            #[cfg(feature = "playback")]
            volume: 0.1,

            #[cfg(feature = "playback")]
            _output_stream: output_stream,
            #[cfg(feature = "playback")]
            output_stream_handle,
            #[cfg(feature = "playback")]
            sinks: VecDeque::new(),

            #[cfg(feature = "memory")]
            fs_memory: Default::default(),
        })
    }

    #[must_use]
    pub fn get(&self, sentence: &str) -> Option<&Vec<Chatsound>> {
        self.map_store.get(sentence)
    }

    #[must_use]
    pub fn search(&self, search: &str) -> Vec<(usize, &String)> {
        #[cfg(feature = "rayon")]
        use rayon::prelude::*;

        #[cfg(feature = "rayon")]
        let iter = HashMap::par_iter;
        #[cfg(not(feature = "rayon"))]
        let iter = HashMap::iter;

        #[cfg(feature = "rayon")]
        let sort_by = <[(usize, &String)]>::par_sort_unstable_by;
        #[cfg(not(feature = "rayon"))]
        let sort_by = <[(usize, &String)]>::sort_unstable_by;

        let mut positions: Vec<_> = iter(&self.map_store)
            .map(|(key, _value)| key)
            .filter_map(|sentence| twoway::find_str(sentence, search).map(|pos| (pos, sentence)))
            .collect();

        sort_by(
            positions.as_mut_slice(),
            |(pos1, str1): &(usize, &String), (pos2, str2): &(usize, &String)| {
                pos1.partial_cmp(pos2)
                    .unwrap()
                    .then_with(|| str1.len().partial_cmp(&str2.len()).unwrap())
            },
        );

        positions
    }

    #[cfg(feature = "playback")]
    pub fn stop_all(&mut self) {
        for sink in self.sinks.drain(..) {
            sink.stop();
        }
    }

    #[cfg(feature = "playback")]
    pub fn set_volume(&mut self, volume: f32) {
        let volume = volume.max(0.0);

        self.volume = volume;

        for sink in &mut self.sinks {
            sink.set_volume(volume);
        }
    }

    #[cfg(feature = "playback")]
    #[must_use]
    pub const fn volume(&self) -> f32 {
        self.volume
    }

    #[cfg(feature = "playback")]
    pub async fn play<R: RngCore>(&mut self, text: &str, rng: R) -> Result<Arc<Sink>> {
        let sink = Arc::new(Sink::try_new(&self.output_stream_handle)?);

        sink.set_volume(self.volume);

        for source in self.get_sources(text, rng).await? {
            sink.append(source);
        }

        self.sinks.push_back(Box::new(sink.clone()));
        if self.sinks.len() == self.max_sinks {
            self.sinks.pop_front();
        }

        Ok(sink)
    }

    #[cfg(feature = "playback")]
    pub async fn play_spatial<R: RngCore>(
        &mut self,
        text: &str,
        rng: R,
        emitter_pos: [f32; 3],
        left_ear_pos: [f32; 3],
        right_ear_pos: [f32; 3],
    ) -> Result<Arc<SpatialSink>> {
        let sink = Arc::new(SpatialSink::try_new(
            &self.output_stream_handle,
            emitter_pos,
            left_ear_pos,
            right_ear_pos,
        )?);

        sink.set_volume(self.volume);

        for source in self.get_sources(text, rng).await? {
            sink.append(source);
        }

        self.sinks.push_back(Box::new(sink.clone()));
        if self.sinks.len() == self.max_sinks {
            self.sinks.pop_front();
        }

        Ok(sink)
    }

    #[cfg(feature = "playback")]
    pub async fn play_channel_volume<R: RngCore>(
        &mut self,
        text: &str,
        rng: R,
        channel_volumes: Vec<f32>,
    ) -> Result<Arc<ChannelVolumeSink>> {
        let sink = Arc::new(ChannelVolumeSink::try_new(
            &self.output_stream_handle,
            channel_volumes,
        )?);

        sink.sink.set_volume(self.volume);

        for source in self.get_sources(text, rng).await? {
            sink.append(source);
        }

        self.sinks.push_back(Box::new(sink.clone()));
        if self.sinks.len() == self.max_sinks {
            self.sinks.pop_front();
        }

        Ok(sink)
    }

    pub async fn get_sources<R: RngCore>(
        &mut self,
        text: &str,
        mut rng: R,
    ) -> Result<Vec<BoxSource>> {
        let mut sources = Vec::new();

        let parsed_chatsounds = parse(text)?;
        for parsed_chatsound in parsed_chatsounds {
            let chatsound = if let Some(chatsounds) = self.get(&parsed_chatsound.sentence) {
                // TODO random hashed number passed in?
                parsed_chatsound
                    .choose(chatsounds, &mut rng)
                    .ok_or_else(|| Error::EmptyChoose { text: text.into() })?
                    .to_owned()
            } else {
                continue;
            };

            let mut source: BoxSource = {
                #[cfg(feature = "fs")]
                let cache = &self.cache_path;
                #[cfg(feature = "memory")]
                let cache = self.fs_memory.clone();

                let bytes = chatsound.load(cache).await?;

                let reader = BufReader::new(Cursor::new(bytes));
                Box::new(Decoder::new(reader).map_err(|err| Error::RodioDecoder {
                    err,
                    sound_path: chatsound.sound_path,
                })?)
            };
            for modifier in parsed_chatsound.modifiers {
                source = modifier.modify(source);
            }

            sources.push(source);
        }

        Ok(sources)
    }

    #[cfg(feature = "playback")]
    pub fn sleep_until_end(&mut self) {
        for sink in self.sinks.drain(..) {
            sink.sleep_until_end();
        }
    }
}
