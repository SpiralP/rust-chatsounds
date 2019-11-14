mod helpers;
mod modifiers;
mod parser;

use crate::helpers::cache_download;
use async_std::sync::Arc;
use async_trait::async_trait;
use bytes::Bytes;
use rayon::prelude::*;
pub use rodio::{Decoder, Device, Sink, SpatialSink};
use rodio::{Sample, Source};
use serde::Deserialize;
use std::{
  collections::{HashMap, VecDeque},
  fmt::Debug,
  io::{BufReader, Cursor},
  path::{Component, Path, PathBuf},
};

#[derive(Deserialize)]
struct GitHubApiTrees {
  pub tree: Vec<GitHubApiFileEntry>,
}

// {
// "path": "sound/chatsounds/autoadd/instagib/sex1.ogg",
// "mode": "100644",
// "type": "blob",
// "sha": "035b008bbf3d63ede0fb49eee52734ffa94321ba",
// "size": 28324,
// "url": "https://api.github.com/repos/Metastruct/garrysmod-chatsounds/git/blobs/035b008bbf3d63ede0fb49eee52734ffa94321ba"
// },
#[derive(Deserialize)]
struct GitHubApiFileEntry {
  pub path: String,
  pub r#type: String,
  pub size: Option<usize>,
}

#[async_trait]
pub trait ChatsoundTrait {
  async fn get_bytes<P: AsRef<Path> + Send + Sync>(&self, cache_path: P) -> Bytes;
}

#[derive(Clone)]
pub struct Chatsound {
  // Metastruct/garrysmod-chatsounds
  repo: String,
  // sound/chatsounds/autoadd
  repo_path: String,
  // 26e/nestetrismusic/1.ogg
  sound_path: String,
  // nestetrismusic
  sentence: String,
}

impl Chatsound {
  pub async fn load<P: AsRef<Path>>(&self, cache_path: P) -> LoadedChatsound {
    let url = format!(
      "https://raw.githubusercontent.com/{}/master/{}/{}",
      self.repo, self.repo_path, self.sound_path
    );

    let bytes = cache_download(&url, cache_path).await;

    LoadedChatsound { bytes }
  }
}

pub struct LoadedChatsound {
  bytes: Bytes,
}

#[async_trait]
impl ChatsoundTrait for LoadedChatsound {
  async fn get_bytes<P: AsRef<Path> + Send + Sync>(&self, _cache_path: P) -> Bytes {
    self.bytes.clone()
  }
}

#[async_trait]
impl ChatsoundTrait for Chatsound {
  async fn get_bytes<P: AsRef<Path> + Send + Sync>(&self, cache_path: P) -> Bytes {
    let loaded_chatsound = self.load(&cache_path).await;
    loaded_chatsound.get_bytes(&cache_path).await
  }
}

#[derive(Clone)]
enum ChatsoundsSink {
  Sink(Arc<Sink>),
  Spatial(Arc<SpatialSink>),
}

impl ChatsoundsSink {
  fn append<S>(&mut self, source: S)
  where
    S: Source + Send + 'static,
    S::Item: Sample + Send + Debug,
  {
    match self {
      ChatsoundsSink::Sink(sink) => sink.append(source),
      ChatsoundsSink::Spatial(sink) => sink.append(source),
    }
  }

  fn sleep_until_end(&mut self) {
    match self {
      ChatsoundsSink::Sink(sink) => sink.sleep_until_end(),
      ChatsoundsSink::Spatial(sink) => sink.sleep_until_end(),
    }
  }

  fn stop(&mut self) {
    match self {
      ChatsoundsSink::Sink(sink) => sink.stop(),
      ChatsoundsSink::Spatial(sink) => sink.stop(),
    }
  }

  fn set_volume(&mut self, volume: f32) {
    match self {
      ChatsoundsSink::Sink(sink) => sink.set_volume(volume),
      ChatsoundsSink::Spatial(sink) => sink.set_volume(volume),
    }
  }

  fn unwrap_sink(self) -> Arc<Sink> {
    match self {
      ChatsoundsSink::Sink(sink) => sink,
      _ => unreachable!(),
    }
  }

  fn unwrap_spatial(self) -> Arc<SpatialSink> {
    match self {
      ChatsoundsSink::Spatial(sink) => sink,
      _ => unreachable!(),
    }
  }
}

pub struct Chatsounds {
  cache_path: PathBuf,
  max_sinks: usize,
  volume: f32,

  // [sentence]: Chatsound[]
  map_store: HashMap<String, Vec<Chatsound>>,

  device: Device,
  sinks: VecDeque<ChatsoundsSink>,
}

impl Chatsounds {
  pub fn new<T: AsRef<Path>>(cache_path: T) -> Self {
    Self {
      cache_path: cache_path.as_ref().canonicalize().unwrap(),
      max_sinks: 8,
      volume: 0.1,
      map_store: HashMap::new(),
      device: rodio::default_output_device().unwrap(),
      sinks: VecDeque::new(),
    }
  }

  // TODO parse HEAD request
  pub async fn load_github_api(&mut self, repo: &str, repo_path: &str) {
    let api_url = format!(
      "https://api.github.com/repos/{}/git/trees/master?recursive=1",
      repo
    );

    let bytes = cache_download(api_url, &self.cache_path).await;
    let mut trees: GitHubApiTrees = serde_json::from_slice(&bytes).unwrap();

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
            sentence,
          });
        }
      }
    }
  }

  pub async fn load_github_msgpack(&mut self, repo: &str, repo_path: &str) {
    let msgpack_url = format!(
      "https://raw.githubusercontent.com/{}/master/{}/list.msgpack",
      repo, repo_path
    );

    let bytes = cache_download(msgpack_url, &self.cache_path).await;
    let mut entries: Vec<Vec<String>> = rmp_serde::decode::from_slice(&bytes).unwrap();

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
        sentence,
      });
    }
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
      pos1
        .partial_cmp(pos2)
        .unwrap()
        .then_with(|| str1.len().partial_cmp(&str2.len()).unwrap())
    });

    positions.par_iter().cloned().collect()
  }

  pub fn stop_all(&mut self) {
    for mut sink in self.sinks.drain(..) {
      sink.stop();
    }
  }

  pub fn set_volume(&mut self, volume: f32) {
    self.volume = volume;

    for sink in &mut self.sinks {
      sink.set_volume(volume);
    }
  }

  pub fn volume(&self) -> f32 {
    self.volume
  }

  pub fn cache_path(&self) -> &PathBuf {
    &self.cache_path
  }

  pub async fn play<S: AsRef<str>>(&mut self, text: S) -> Result<Arc<Sink>, String> {
    let mut sink = ChatsoundsSink::Sink(Arc::new(Sink::new(&self.device)));

    self.play_sink(text, &mut sink).await?;

    Ok(sink.unwrap_sink())
  }

  pub async fn play_spatial<S: AsRef<str>>(
    &mut self,
    text: S,
    emitter_pos: [f32; 3],
    left_ear_pos: [f32; 3],
    right_ear_pos: [f32; 3],
  ) -> Result<Arc<SpatialSink>, String> {
    let mut sink = ChatsoundsSink::Spatial(Arc::new(SpatialSink::new(
      &self.device,
      emitter_pos,
      left_ear_pos,
      right_ear_pos,
    )));

    self.play_sink(text, &mut sink).await?;

    Ok(sink.unwrap_spatial())
  }

  async fn play_sink<S: AsRef<str>>(
    &mut self,
    text: S,
    sink: &mut ChatsoundsSink,
  ) -> Result<(), String> {
    sink.set_volume(self.volume);

    let parsed_chatsounds = parser::parse(text.as_ref())?;
    for parsed_chatsound in parsed_chatsounds {
      if let Some(chatsounds) = self.get(parsed_chatsound.sentence) {
        // TODO random hashed number passed in?
        let chatsound = chatsounds.get(0).unwrap();

        let mut source: Box<dyn Source<Item = i16> + Send> =
          Box::new(self.make_source(chatsound).await);

        for modifier in parsed_chatsound.modifiers {
          source = modifier.modify(source);
        }

        sink.append(source);
      }
    }

    self.sinks.push_back(sink.clone());
    if self.sinks.len() == self.max_sinks {
      self.sinks.pop_front();
    }

    Ok(())
  }

  async fn make_source<C: ChatsoundTrait>(
    &self,
    chatsound: &C,
  ) -> Decoder<BufReader<Cursor<Bytes>>> {
    let bytes = chatsound.get_bytes(&self.cache_path).await;

    let reader = BufReader::new(Cursor::new(bytes));
    Decoder::new(reader).unwrap()
  }

  pub fn sleep_until_end(&mut self) {
    for mut sink in self.sinks.drain(..) {
      sink.sleep_until_end();
    }
  }
}

#[cfg(test)]
mod tests {

  use super::*;
  use futures::prelude::Future;

  fn run_future<F: Future<Output = ()>>(f: F) {
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(f);
  }

  #[test]
  fn it_works() {
    use async_std::fs;

    run_future(async {
      fs::create_dir_all("cache").await.unwrap();

      let mut chatsounds = Chatsounds::new("cache");

      println!("Metastruct/garrysmod-chatsounds");
      chatsounds
        .load_github_api(
          "Metastruct/garrysmod-chatsounds",
          "sound/chatsounds/autoadd",
        )
        .await;

      println!("PAC3-Server/chatsounds");
      chatsounds
        .load_github_api("PAC3-Server/chatsounds", "sounds/chatsounds")
        .await;

      for folder in &[
        "csgo", "css", "ep1", "ep2", "hl2", "l4d", "l4d2", "portal", "tf2",
      ] {
        println!("PAC3-Server/chatsounds-valve-games {}", folder);
        chatsounds
          .load_github_msgpack("PAC3-Server/chatsounds-valve-games", folder)
          .await;
      }

      chatsounds
        .play("helloh:speed(1) im gay:speed(1.2):echo(0.5,0.2) dad please:speed(0.5)")
        .await
        .unwrap()
        .sleep_until_end();
    });
  }

  // #[test]
  // fn test_autocomplete() {
  //   run_future(async {
  //     let chatsounds = {
  //       fs::create_dir_all("cache").await.unwrap();

  //       let mut chatsounds = Chatsounds::new("cache");

  //       println!("Metastruct/garrysmod-chatsounds");
  //       chatsounds
  //         .load_github_api(
  //           "Metastruct/garrysmod-chatsounds",
  //           "sound/chatsounds/autoadd",
  //         )
  //         .await;

  //       println!("PAC3-Server/chatsounds");
  //       chatsounds
  //         .load_github_api("PAC3-Server/chatsounds", "sounds/chatsounds")
  //         .await;

  //       for folder in &[
  //         "csgo", "css", "ep1", "ep2", "hl2", "l4d", "l4d2", "portal", "tf2",
  //       ] {
  //         println!("PAC3-Server/chatsounds-valve-games {}", folder);
  //         chatsounds
  //           .load_github_msgpack("PAC3-Server/chatsounds-valve-games", folder)
  //           .await;
  //       }

  //       chatsounds
  //     };

  //     println!("searching {} keys", chatsounds.map_store.keys().count());
  //     let search = "and thats what";

  //     let t0 = std::time::Instant::now();
  //     let positions = chatsounds.search(search.to_string());
  //     let t1 = std::time::Instant::now();
  //     println!("took {:?}", t1 - t0);

  //     println!(
  //       "{:#?}",
  //       positions.iter().rev().take(10).rev().collect::<Vec<_>>()
  //     );
  //   });
  // }

  // #[test]
  // fn test_spatial() {
  //   run_future(async {
  //     fs::create_dir_all("cache").await.unwrap();

  //     let mut chatsounds = Chatsounds::new("cache");

  //     println!("fetch");
  //     chatsounds
  //       .load_github_api(
  //         "Metastruct/garrysmod-chatsounds",
  //         "sound/chatsounds/autoadd",
  //       )
  //       .await;

  //     if let Some(sounds) = chatsounds.get("fuckbeesremastered") {
  //       if let Some(chatsound) = sounds.get(0).cloned() {
  //         // play near right ear
  //         let emitter_pos = [2.0, 0.0, 0.0];
  //         let left_ear_pos = [-1.0, 0.0, 0.0];
  //         let right_ear_pos = [1.0, 0.0, 0.0];

  //         println!("play");

  //         let sink = chatsounds
  //           .play_spatial(&chatsound, emitter_pos, left_ear_pos, right_ear_pos)
  //           .await;

  //         while !sink.empty() {
  //           std::thread::sleep(std::time::Duration::from_millis(10));
  //           sink.set_left_ear_position(left_ear_pos);
  //           sink.set_right_ear_position(right_ear_pos);
  //           sink.set_emitter_position(emitter_pos);
  //         }

  //         sink.sleep_until_end();
  //       }
  //     }

  //     chatsounds.sleep_until_end();
  //   });
  // }

  // #[test]
  // fn test_mono_bug() {
  //   run_future(async {
  //     fs::create_dir_all("cache").await.unwrap();

  //     let mut chatsounds = Chatsounds::new("cache");

  //     println!("fetch");
  //     chatsounds
  //       .load_github_api(
  //         "Metastruct/garrysmod-chatsounds",
  //         "sound/chatsounds/autoadd",
  //       )
  //       .await;

  //     if let Some(sounds) = chatsounds.get("fuckbeesremastered") {
  //       if let Some(chatsound) = sounds.get(0).cloned() {
  //         println!("play");

  //         let data = chatsound.get_bytes(&chatsounds.cache_path).await;

  //         let reader = BufReader::new(Cursor::new(data));
  //         let source = Decoder::new(reader).unwrap();

  //         let source = rodio::source::ChannelVolume::new(source, vec![0.2, 1.0]);
  //         let sink = Sink::new(&chatsounds.device);
  //         sink.set_volume(0.1);
  //         sink.append(source);

  //         sink.sleep_until_end();
  //       }
  //     }

  //     chatsounds.sleep_until_end();
  //   });
  // }
}
