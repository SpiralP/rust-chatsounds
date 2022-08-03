mod helpers;
mod modifiers;
mod parser;

use std::{
    collections::{HashMap, VecDeque},
    fs,
    io::{BufReader, Cursor},
    path::{Component, Path, PathBuf},
    sync::Arc,
};

use anyhow::{anyhow, bail, Context, Result};
use async_trait::async_trait;
use bytes::Bytes;
use rand::prelude::*;
use rayon::prelude::*;
pub use rodio::{queue::SourcesQueueOutput, Decoder, Device, Sample, Sink, Source, SpatialSink};
use rodio::{OutputStream, OutputStreamHandle};
use serde::Deserialize;

use self::{helpers::cache_download, modifiers::ModifierTrait};

pub type BoxSource = Box<dyn Source<Item = i16> + Send>;

#[derive(Deserialize)]
pub struct GitHubApiTrees {
    pub tree: Vec<GitHubApiFileEntry>,
}

pub type GitHubMsgpackEntries = Vec<Vec<String>>;

// {
// "path": "sound/chatsounds/autoadd/instagib/sex1.ogg",
// "mode": "100644",
// "type": "blob",
// "sha": "035b008bbf3d63ede0fb49eee52734ffa94321ba",
// "size": 28324,
// "url": "https://api.github.com/repos/Metastruct/garrysmod-chatsounds/git/blobs/035b008bbf3d63ede0fb49eee52734ffa94321ba"
// },
#[derive(Deserialize)]
pub struct GitHubApiFileEntry {
    pub path: String,
    pub r#type: String,
    pub size: Option<usize>,
}

#[async_trait]
pub trait ChatsoundTrait {
    async fn get_bytes(&self, cache_path: &Path) -> Result<Bytes>;
}

#[derive(Clone)]
pub struct Chatsound {
    // Metastruct/garrysmod-chatsounds
    repo: String,
    // sound/chatsounds/autoadd
    repo_path: String,
    // 26e/nestetrismusic/1.ogg
    sound_path: String,
}

impl Chatsound {
    pub async fn load(&self, cache_path: &Path) -> Result<LoadedChatsound> {
        let url = format!(
            "https://raw.githubusercontent.com/{}/HEAD/{}/{}",
            self.repo, self.repo_path, self.sound_path
        );

        let bytes = cache_download(&url, cache_path, false, |bytes| Ok(Ok(bytes))).await?;

        Ok(LoadedChatsound { bytes })
    }
}

pub struct LoadedChatsound {
    bytes: Bytes,
}

#[async_trait]
impl ChatsoundTrait for LoadedChatsound {
    async fn get_bytes(&self, _cache_path: &Path) -> Result<Bytes> {
        Ok(self.bytes.clone())
    }
}

#[async_trait]
impl ChatsoundTrait for Chatsound {
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

pub struct Chatsounds {
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
}

unsafe impl Send for Chatsounds {}
unsafe impl Sync for Chatsounds {}

impl Chatsounds {
    pub fn new<T: AsRef<Path>>(cache_path: T) -> Result<Self> {
        let cache_path = cache_path.as_ref().canonicalize()?;
        if !fs::metadata(&cache_path)
            .map(|meta| meta.is_dir())
            .unwrap_or(false)
        {
            bail!("cache_path doesn't exist!");
        }

        #[cfg(feature = "playback")]
        let (output_stream, output_stream_handle) =
            OutputStream::try_default().context("OutputStream::try_default")?;

        Ok(Self {
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
        })
    }

    pub async fn fetch_github_api(
        &self,
        repo: &str,
        _repo_path: &str,
        use_etag: bool,
    ) -> Result<GitHubApiTrees> {
        let api_url = format!(
            "https://api.github.com/repos/{}/git/trees/HEAD?recursive=1",
            repo
        );

        let bytes = cache_download(&api_url, self.cache_path(), use_etag, |bytes| {
            #[derive(Deserialize)]
            struct GitHubError {
                message: String,
            }

            Ok(
                if let Ok(err) = serde_json::from_slice::<GitHubError>(&bytes) {
                    let error = anyhow!("GitHub Error: {}", err.message);
                    if err.message.starts_with("API rate limit exceeded") {
                        Err(error)
                    } else {
                        bail!(error);
                    }
                } else {
                    Ok(bytes)
                },
            )
        })
        .await?;

        let trees: GitHubApiTrees = serde_json::from_slice(&bytes)?;

        Ok(trees)
    }

    pub fn load_github_api(
        &mut self,
        repo: &str,
        repo_path: &str,
        mut trees: GitHubApiTrees,
    ) -> Result<()> {
        for entry in trees.tree.iter_mut() {
            if entry.r#type != "blob" || !entry.path.starts_with(&repo_path) {
                continue;
            }

            // e26/stop.ogg or e26/nestetrismusic/1.ogg
            let sound_path = entry.path.split_off(repo_path.len() + 1);

            let path = Path::new(&sound_path);
            if let Some(Component::Normal(s)) = path.components().nth(1) {
                if let Some(sentence) = Path::new(&s).file_stem() {
                    let sentence = sentence.to_string_lossy().to_string();
                    let vec = self
                        .map_store
                        .entry(sentence.to_string())
                        .or_insert_with(Vec::new);
                    vec.push(Chatsound {
                        repo: repo.to_string(),
                        repo_path: repo_path.to_string(),
                        sound_path,
                    });
                }
            }
        }

        Ok(())
    }

    pub async fn fetch_github_msgpack(
        &self,
        repo: &str,
        repo_path: &str,
        use_etag: bool,
    ) -> Result<GitHubMsgpackEntries> {
        let msgpack_url = format!(
            "https://raw.githubusercontent.com/{}/HEAD/{}/list.msgpack",
            repo, repo_path
        );

        // these raw links don't have a rate limit so we won't cache bad results
        let bytes = cache_download(&msgpack_url, self.cache_path(), use_etag, |bytes| {
            Ok(Ok(bytes))
        })
        .await?;
        let entries: GitHubMsgpackEntries = rmp_serde::decode::from_slice(&bytes)?;

        Ok(entries)
    }

    pub fn load_github_msgpack(
        &mut self,
        repo: &str,
        repo_path: &str,
        mut entries: GitHubMsgpackEntries,
    ) -> Result<()> {
        for entry in entries.drain(..) {
            // e26/stop.ogg or e26/nestetrismusic/1.ogg
            let sentence = entry[1].clone();
            let sound_path = entry[2].clone();
            let vec = self
                .map_store
                .entry(sentence.to_string())
                .or_insert_with(Vec::new);

            vec.push(Chatsound {
                repo: repo.to_string(),
                repo_path: repo_path.to_string(),
                sound_path,
            });
        }

        Ok(())
    }

    pub fn get<T: AsRef<str>>(&self, sentence: T) -> Option<&Vec<Chatsound>> {
        self.map_store.get(sentence.as_ref())
    }

    pub fn search<S: AsRef<str>>(&self, search: S) -> Vec<(usize, &String)> {
        let search = search.as_ref();

        let mut positions: Vec<_> = self
            .map_store
            .par_iter()
            .map(|(key, _value)| key)
            .filter_map(|sentence| twoway::find_str(sentence, search).map(|pos| (pos, sentence)))
            .collect();

        positions.par_sort_unstable_by(|(pos1, str1), (pos2, str2)| {
            pos1.partial_cmp(pos2)
                .unwrap()
                .then_with(|| str1.len().partial_cmp(&str2.len()).unwrap())
        });

        positions.par_iter().cloned().collect()
    }

    #[cfg(feature = "playback")]
    pub fn stop_all(&mut self) {
        for mut sink in self.sinks.drain(..) {
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
    pub fn volume(&self) -> f32 {
        self.volume
    }

    pub fn cache_path(&self) -> &PathBuf {
        &self.cache_path
    }

    #[cfg(feature = "playback")]
    pub async fn play<R: RngCore>(&mut self, text: &str, rng: R) -> Result<Arc<Sink>> {
        let mut sink = Arc::new(Sink::try_new(&self.output_stream_handle)?);

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
        let mut sink = Arc::new(SpatialSink::try_new(
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

    pub async fn get_sources<R>(&mut self, text: &str, mut rng: R) -> Result<Vec<BoxSource>>
    where
        R: RngCore,
    {
        let mut sources = Vec::new();

        let parsed_chatsounds = parser::parse(text)?;
        for parsed_chatsound in parsed_chatsounds {
            if let Some(chatsounds) = self.get(parsed_chatsound.sentence) {
                // TODO random hashed number passed in?
                let chatsound = chatsounds.choose(&mut rng).unwrap();

                let mut source: BoxSource = {
                    let bytes = chatsound.get_bytes(&self.cache_path).await?;
                    let reader = BufReader::new(Cursor::new(bytes));
                    Box::new(Decoder::new(reader)?)
                };
                for modifier in parsed_chatsound.modifiers {
                    source = modifier.modify(source);
                }

                sources.push(source);
            }
        }

        Ok(sources)
    }

    #[cfg(feature = "playback")]
    pub fn sleep_until_end(&mut self) {
        for mut sink in self.sinks.drain(..) {
            sink.sleep_until_end();
        }
    }
}

#[cfg(test)]
mod tests {
    use futures::{
        future::{FutureExt, TryFutureExt},
        stream::{StreamExt, TryStreamExt},
    };
    use tokio::fs;

    use super::*;

    enum Source {
        Api(&'static str, &'static str),
        Msgpack(&'static str, &'static str),
    }

    const SOURCES: &[Source] = &[
        Source::Api("NotAwesome2/chatsounds", "sounds"),
        Source::Api(
            "Metastruct/garrysmod-chatsounds",
            "sound/chatsounds/autoadd",
        ),
        Source::Api("PAC3-Server/chatsounds", "sounds/chatsounds"),
        Source::Msgpack("PAC3-Server/chatsounds-valve-games", "csgo"),
        Source::Msgpack("PAC3-Server/chatsounds-valve-games", "css"),
        Source::Msgpack("PAC3-Server/chatsounds-valve-games", "ep1"),
        Source::Msgpack("PAC3-Server/chatsounds-valve-games", "ep2"),
        Source::Msgpack("PAC3-Server/chatsounds-valve-games", "hl1"),
        Source::Msgpack("PAC3-Server/chatsounds-valve-games", "hl2"),
        Source::Msgpack("PAC3-Server/chatsounds-valve-games", "l4d"),
        Source::Msgpack("PAC3-Server/chatsounds-valve-games", "l4d2"),
        Source::Msgpack("PAC3-Server/chatsounds-valve-games", "portal"),
        Source::Msgpack("PAC3-Server/chatsounds-valve-games", "tf2"),
    ];

    async fn setup() -> Chatsounds {
        fs::create_dir_all("cache").await.unwrap();

        let mut chatsounds = Chatsounds::new("cache").unwrap();

        {
            enum FetchedSource {
                Api(GitHubApiTrees),
                Msgpack(GitHubMsgpackEntries),
            }

            println!("fetching sources");
            let fetched = futures::stream::iter(SOURCES)
                .map(|source| {
                    let f = match source {
                        Source::Api(repo, repo_path) => chatsounds
                            .fetch_github_api(repo, repo_path, true)
                            .map_ok(FetchedSource::Api)
                            .boxed(),

                        Source::Msgpack(repo, repo_path) => chatsounds
                            .fetch_github_msgpack(repo, repo_path, true)
                            .map_ok(FetchedSource::Msgpack)
                            .boxed(),
                    };

                    async move { f.await.map(|fetched_source| (source, fetched_source)) }
                })
                .buffered(5)
                .try_collect::<Vec<_>>()
                .await
                .unwrap();

            println!("done");

            for (source, fetched_source) in fetched {
                match source {
                    Source::Api(repo, repo_path) => {
                        if let FetchedSource::Api(data) = fetched_source {
                            chatsounds.load_github_api(repo, repo_path, data).unwrap();
                        }
                    }

                    Source::Msgpack(repo, repo_path) => {
                        if let FetchedSource::Msgpack(data) = fetched_source {
                            chatsounds
                                .load_github_msgpack(repo, repo_path, data)
                                .unwrap();
                        }
                    }
                }
            }
        }
        chatsounds
    }

    #[ignore]
    #[tokio::test]
    async fn negative_pitch() {
        let mut chatsounds = setup().await;

        chatsounds
            .play("helloh:speed(1) musicbox:pitch(-1)", thread_rng())
            .await
            .unwrap()
            .sleep_until_end();
    }

    #[ignore]
    #[tokio::test]
    async fn it_works() {
        let mut chatsounds = setup().await;

        chatsounds
            .play(
                "helloh:speed(1) idubbbz cringe:speed(1.2):echo(0.5,0.2) dad please:speed(0.5)",
                thread_rng(),
            )
            .await
            .unwrap()
            .sleep_until_end();
    }

    #[tokio::test]
    async fn test_autocomplete() {
        let chatsounds = setup().await;

        println!("searching {} keys", chatsounds.map_store.keys().count());
        let search = "and thats what";

        let t0 = std::time::Instant::now();
        let positions = chatsounds.search(search);
        let t1 = std::time::Instant::now();
        println!("took {:?}", t1 - t0);

        println!(
            "{:#?}",
            positions.iter().rev().take(10).rev().collect::<Vec<_>>()
        );
    }

    #[ignore]
    #[tokio::test]
    async fn test_spatial() {
        let mut chatsounds = setup().await;

        let mut emitter_pos = [2.0, 0.0, 0.0];
        let left_ear_pos = [-1.0, 0.0, 0.0];
        let right_ear_pos = [1.0, 0.0, 0.0];

        println!("play");
        let mut sink = chatsounds
            .play_spatial(
                "fuckbeesremastered",
                thread_rng(),
                emitter_pos,
                left_ear_pos,
                right_ear_pos,
            )
            .await
            .unwrap();

        // start in right headphone, move left
        let mut i: f32 = 0.0;
        while !sink.empty() {
            emitter_pos[0] = i.cos() * 2.0;

            sink.set_left_ear_position(left_ear_pos);
            sink.set_right_ear_position(right_ear_pos);
            sink.set_emitter_position(emitter_pos);

            std::thread::sleep(std::time::Duration::from_millis(10));
            i += 0.01;
        }

        sink.sleep_until_end();

        chatsounds.sleep_until_end();
    }

    #[ignore]
    #[tokio::test]
    async fn test_mono_bug() {
        let mut chatsounds = setup().await;

        let sounds = chatsounds.get("mktheme").unwrap();
        if let Some(chatsound) = sounds.get(0).cloned() {
            println!("play");

            let data = chatsound.get_bytes(&chatsounds.cache_path).await.unwrap();

            let reader = BufReader::new(Cursor::new(data));
            let source = Decoder::new(reader).unwrap();

            let source = rodio::source::ChannelVolume::new(source, vec![0.2, 1.0]);
            let sink = Sink::try_new(&chatsounds.output_stream_handle).unwrap();
            sink.set_volume(0.1);
            sink.append(source);

            sink.sleep_until_end();
        }

        chatsounds.sleep_until_end();
    }

    #[tokio::test]
    async fn test_get_samples() {
        let mut chatsounds = setup().await;

        let mut sources = chatsounds
            .get_sources("mktheme", thread_rng())
            .await
            .unwrap();

        let (sink, queue) = rodio::queue::queue(false);
        for source in sources.drain(..) {
            sink.append(source);
        }

        let queue: rodio::source::UniformSourceIterator<SourcesQueueOutput<i16>, i16> =
            rodio::source::UniformSourceIterator::new(queue, 2, 44100);

        println!("{} samples", queue.count());
    }
}
