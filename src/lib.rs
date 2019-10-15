use bytes::Bytes;
use rayon::prelude::*;
pub use rodio::{Decoder, Device, Sink, SpatialSink};
use serde::Deserialize;
use sha2::{Digest, Sha256};
use std::{
  collections::{HashMap, VecDeque},
  fs,
  fs::{File, OpenOptions},
  io,
  io::{prelude::*, BufReader, Cursor},
  path::{Component, Path, PathBuf},
  sync::Arc,
};

fn cache_download<S: AsRef<str>, P: AsRef<Path>>(url: S, cache_path: P) -> impl Read {
  let mut hasher = Sha256::new();
  hasher.input(url.as_ref());

  let hex = format!("{:x}", hasher.result());

  let hex_dir = &hex[0..2];
  let hex_filename = &hex[2..];

  let dir_path = cache_path.as_ref().join(hex_dir);
  if !fs::metadata(&dir_path)
    .map(|meta| meta.is_dir())
    .unwrap_or(false)
  {
    fs::create_dir(&dir_path).unwrap();
  }

  let file_path = dir_path.join(hex_filename);
  if !fs::metadata(&file_path)
    .map(|meta| meta.is_file())
    .unwrap_or(false)
  {
    let mut file = File::create(&file_path).unwrap();
    let mut response = reqwest::get(url.as_ref()).unwrap();
    io::copy(&mut response, &mut file).unwrap();
  }

  let file = OpenOptions::new().read(true).open(&file_path).unwrap();
  BufReader::new(file)
}

#[derive(Deserialize)]
struct GitHubApiTrees {
  pub tree: Vec<GitHubApiFileEntry>,
}

/*
    {
      "path": "sound/chatsounds/autoadd/instagib/sex1.ogg",
      "mode": "100644",
      "type": "blob",
      "sha": "035b008bbf3d63ede0fb49eee52734ffa94321ba",
      "size": 28324,
      "url": "https://api.github.com/repos/Metastruct/garrysmod-chatsounds/git/blobs/035b008bbf3d63ede0fb49eee52734ffa94321ba"
    },
*/
#[derive(Deserialize)]
struct GitHubApiFileEntry {
  pub path: String,
  pub r#type: String,
  pub size: Option<usize>,
}

pub trait ChatsoundTrait {
  fn get_bytes<P: AsRef<Path>>(&self, cache_path: P) -> Bytes;
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
  pub fn load<P: AsRef<Path>>(&self, cache_path: P) -> LoadedChatsound {
    let url = format!(
      "https://raw.githubusercontent.com/{}/master/{}/{}",
      self.repo, self.repo_path, self.sound_path
    );

    let mut response = cache_download(&url, cache_path);

    let mut bytes = Vec::new();
    response.read_to_end(&mut bytes).unwrap();
    let bytes = Bytes::from(bytes);

    LoadedChatsound { bytes }
  }
}

pub struct LoadedChatsound {
  bytes: Bytes,
}

impl ChatsoundTrait for LoadedChatsound {
  fn get_bytes<P: AsRef<Path>>(&self, _cache_path: P) -> Bytes {
    self.bytes.clone()
  }
}

impl ChatsoundTrait for Chatsound {
  fn get_bytes<P: AsRef<Path>>(&self, cache_path: P) -> Bytes {
    let loaded_chatsound = self.load(&cache_path);
    loaded_chatsound.get_bytes(&cache_path)
  }
}

pub struct Chatsounds {
  cache_path: PathBuf,
  max_sinks: usize,
  volume: f32,

  // [sentence]: Chatsound[]
  map_store: HashMap<String, Vec<Chatsound>>,

  device: Device,
  sinks: VecDeque<Arc<Sink>>,
  spatial_sinks: VecDeque<Arc<SpatialSink>>,
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
      spatial_sinks: VecDeque::new(),
    }
  }

  pub fn load_github_api(&mut self, repo: &'static str, repo_path: &'static str) {
    let api_url = format!(
      "https://api.github.com/repos/{}/git/trees/master?recursive=1",
      repo
    );

    let mut response = cache_download(api_url, &self.cache_path);
    let mut trees: GitHubApiTrees = serde_json::from_reader(&mut response).unwrap();

    for entry in trees.tree.iter_mut() {
      if entry.r#type != "blob" || !entry.path.starts_with(&repo_path) {
        continue;
      }

      // e26/stop.ogg or e26/nestetrismusic/1.ogg
      let sound_path = entry.path.split_off(repo_path.len() + 1);

      let path = Path::new(&sound_path);
      if let Some(Component::Normal(s)) = path.components().nth(1) {
        if let Some(sentence) = Path::new(&s).file_stem() {
          // TODO less cloning, maybe "atom"? or Cow or Rc?
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

  pub fn load_github_msgpack(&mut self, repo: &'static str, repo_path: &'static str) {
    let msgpack_url = format!(
      "https://raw.githubusercontent.com/{}/master/{}/list.msgpack",
      repo, repo_path
    );

    let response = cache_download(msgpack_url, &self.cache_path);
    let mut entries: Vec<Vec<String>> = rmp_serde::decode::from_read(response).unwrap();

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

  pub fn get<'a, T: AsRef<str>>(&'a self, sentence: T) -> Option<&'a Vec<Chatsound>> {
    self.map_store.get(sentence.as_ref())
  }

  pub fn search<'a, S: AsRef<str>>(&'a self, search: S) -> Vec<(usize, &'a String)> {
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
    for sink in self.sinks.drain(..) {
      sink.stop();
    }
    for sink in self.spatial_sinks.drain(..) {
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

  pub fn play<C: ChatsoundTrait>(&mut self, chatsound: &C) -> Arc<Sink> {
    let data = chatsound.get_bytes(&self.cache_path);

    let reader = BufReader::new(Cursor::new(data));
    let source = Decoder::new(reader).unwrap();
    let sink = Sink::new(&self.device);
    sink.set_volume(self.volume);
    sink.append(source);

    let sink = Arc::new(sink);
    self.sinks.push_back(sink.clone());
    if self.sinks.len() == self.max_sinks {
      self.sinks.pop_front();
    }

    sink
  }

  pub fn play_spatial<C: ChatsoundTrait>(
    &mut self,
    chatsound: &C,
    emitter_pos: [f32; 3],
    left_ear_pos: [f32; 3],
    right_ear_pos: [f32; 3],
  ) -> Arc<SpatialSink> {
    let data = chatsound.get_bytes(&self.cache_path);

    let reader = BufReader::new(Cursor::new(data));
    let source = Decoder::new(reader).unwrap();

    let sink = SpatialSink::new(&self.device, emitter_pos, left_ear_pos, right_ear_pos);
    sink.set_volume(self.volume);
    sink.append(source);

    let sink = Arc::new(sink);
    self.spatial_sinks.push_back(sink.clone());
    if self.spatial_sinks.len() == self.max_sinks {
      self.spatial_sinks.pop_front();
    }

    sink
  }

  pub fn sleep_until_end(&mut self) {
    for sink in self.sinks.drain(..) {
      sink.sleep_until_end();
    }

    for spatial_sink in self.spatial_sinks.drain(..) {
      spatial_sink.sleep_until_end();
    }
  }
}

#[test]
fn it_works() {
  fs::create_dir_all("cache").unwrap();

  let mut chatsounds = Chatsounds::new("cache");

  println!("Metastruct/garrysmod-chatsounds");
  chatsounds.load_github_api(
    "Metastruct/garrysmod-chatsounds",
    "sound/chatsounds/autoadd",
  );

  println!("PAC3-Server/chatsounds");
  chatsounds.load_github_api("PAC3-Server/chatsounds", "sounds/chatsounds");

  for folder in &[
    "csgo", "css", "ep1", "ep2", "hl2", "l4d", "l4d2", "portal", "tf2",
  ] {
    println!("PAC3-Server/chatsounds-valve-games {}", folder);
    chatsounds.load_github_msgpack("PAC3-Server/chatsounds-valve-games", folder);
  }

  let mut list: Vec<_> = vec!["helloh", "im gay", "dad please"]
    .drain(..)
    .filter_map(|sentence| chatsounds.get(sentence).and_then(|sounds| sounds.get(0)))
    .collect();

  println!("load");
  let mut loaded_list: Vec<_> = list
    .drain(..)
    .map(|chatsound| chatsound.load(chatsounds.cache_path()))
    .collect();

  println!("play");
  for chatsound in loaded_list.drain(..) {
    let sink = chatsounds.play(&chatsound);
    sink.sleep_until_end();
  }
}

#[test]
fn test_autocomplete() {
  let chatsounds = {
    fs::create_dir_all("cache").unwrap();

    let mut chatsounds = Chatsounds::new("cache");

    println!("Metastruct/garrysmod-chatsounds");
    chatsounds.load_github_api(
      "Metastruct/garrysmod-chatsounds",
      "sound/chatsounds/autoadd",
    );

    println!("PAC3-Server/chatsounds");
    chatsounds.load_github_api("PAC3-Server/chatsounds", "sounds/chatsounds");

    for folder in &[
      "csgo", "css", "ep1", "ep2", "hl2", "l4d", "l4d2", "portal", "tf2",
    ] {
      println!("PAC3-Server/chatsounds-valve-games {}", folder);
      chatsounds.load_github_msgpack("PAC3-Server/chatsounds-valve-games", folder);
    }

    chatsounds
  };

  println!("searching {} keys", chatsounds.map_store.keys().count());
  let search = "and thats what";

  let t0 = std::time::Instant::now();
  let positions = chatsounds.search(search.to_string());
  let t1 = std::time::Instant::now();
  println!("took {:?}", t1 - t0);

  println!(
    "{:#?}",
    positions.iter().rev().take(10).rev().collect::<Vec<_>>()
  );
}

#[test]
fn test_spatial() {
  fs::create_dir_all("cache").unwrap();

  let mut chatsounds = Chatsounds::new("cache");
  println!("fetch");
  chatsounds.load_github_api(
    "Metastruct/garrysmod-chatsounds",
    "sound/chatsounds/autoadd",
  );

  if let Some(sounds) = chatsounds.get("fuckbeesremastered") {
    if let Some(chatsound) = sounds.get(0).cloned() {
      // play near right ear
      let emitter_pos = [2.0, 0.0, 0.0];
      let left_ear_pos = [-1.0, 0.0, 0.0];
      let right_ear_pos = [1.0, 0.0, 0.0];

      println!("play");

      let sink = chatsounds.play_spatial(&chatsound, emitter_pos, left_ear_pos, right_ear_pos);

      while !sink.empty() {
        std::thread::sleep(std::time::Duration::from_millis(10));
        sink.set_left_ear_position(left_ear_pos);
        sink.set_right_ear_position(right_ear_pos);
        sink.set_emitter_position(emitter_pos);
      }

      sink.sleep_until_end();
    }
  }

  chatsounds.sleep_until_end();
}

#[test]
fn test_mono_bug() {
  fs::create_dir_all("cache").unwrap();

  let mut chatsounds = Chatsounds::new("cache");
  println!("fetch");
  chatsounds.load_github_api(
    "Metastruct/garrysmod-chatsounds",
    "sound/chatsounds/autoadd",
  );

  if let Some(sounds) = chatsounds.get("fuckbeesremastered") {
    if let Some(chatsound) = sounds.get(0).cloned() {
      println!("play");

      let data = chatsound.get_bytes(&chatsounds.cache_path);

      let reader = BufReader::new(Cursor::new(data));
      let source = Decoder::new(reader).unwrap();

      let source = rodio::source::ChannelVolume::new(source, vec![0.5, 1.0]);
      let sink = Sink::new(&chatsounds.device);
      sink.set_volume(0.1);
      sink.append(source);

      sink.sleep_until_end();
    }
  }

  chatsounds.sleep_until_end();
}
