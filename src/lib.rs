use rodio::{Decoder, Device, Sink};
use serde::Deserialize;
use std::{
  collections::HashMap,
  io::{prelude::*, BufReader, Cursor},
  path::{Component, Path},
};

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

struct ChatsoundsStore {
  repo: String,
  repo_path: String,
  /// [sound_name]: sound_path[]
  store: HashMap<String, Vec<String>>,
}

pub struct Chatsounds {
  stores: Vec<ChatsoundsStore>,

  pub device: Device,
  pub sinks: Vec<Sink>,
  max_sinks: usize,
}

impl Default for Chatsounds {
  fn default() -> Self {
    Self::new()
  }
}

impl Chatsounds {
  /// will download list of files from github `repo`, filtering to `repo_path`
  pub fn new() -> Self {
    Self {
      device: rodio::default_output_device().unwrap(),
      stores: Vec::new(),
      sinks: Vec::new(),
      max_sinks: 2,
    }
  }

  pub fn load_github_api(&mut self, repo: String, repo_path: String) {
    let api_url = format!(
      "https://api.github.com/repos/{}/git/trees/master?recursive=1",
      repo
    );

    let mut store: HashMap<String, Vec<String>> = HashMap::new();

    let mut response = reqwest::get(&api_url).unwrap();
    let mut trees: GitHubApiTrees = response.json().unwrap();

    for entry in trees.tree.iter_mut() {
      if entry.r#type != "blob" || !entry.path.starts_with(&repo_path) {
        continue;
      }

      // e26/stop.ogg or e26/nestetrismusic/1.ogg
      let sound_path = entry.path.split_off(repo_path.len() + 1);

      let path = Path::new(&sound_path);
      if let Some(Component::Normal(s)) = path.components().nth(1) {
        if let Some(name) = Path::new(&s).file_stem() {
          let name = name.to_string_lossy().to_string();
          let vec = store.entry(name).or_insert_with(Vec::new);
          vec.push(sound_path);
        }
      }
    }
    self.stores.push(ChatsoundsStore {
      repo,
      repo_path,
      store,
    });
  }

  pub fn load_github_msgpack(&mut self, repo: String, repo_path: String) {
    let msgpack_url = format!(
      "https://raw.githubusercontent.com/{}/master/{}/list.msgpack",
      repo, repo_path
    );

    let mut store: HashMap<String, Vec<String>> = HashMap::new();

    let response = reqwest::get(&msgpack_url).unwrap();
    let mut entries: Vec<Vec<String>> = rmp_serde::decode::from_read(response).unwrap();

    for entry in entries.drain(..) {
      // e26/stop.ogg or e26/nestetrismusic/1.ogg
      let name = entry[1].clone();
      let sound_path = entry[2].clone();
      let vec = store.entry(name).or_insert_with(Vec::new);
      vec.push(sound_path);
    }

    self.stores.push(ChatsoundsStore {
      repo,
      repo_path,
      store,
    });
  }

  pub fn find<T: Into<String>>(&mut self, search: T) -> Vec<Chatsound> {
    let search = search.into();

    let mut found = Vec::new();

    for store in &self.stores {
      if let Some(sound_paths) = store.store.get(&search) {
        for sound_path in sound_paths {
          let download_url = format!(
            "https://raw.githubusercontent.com/{}/master/{}/{}",
            store.repo, store.repo_path, sound_path
          );

          found.push(Chatsound::from_url(download_url))
        }
      }
    }

    found
  }

  pub fn stop_all(&mut self) {
    for sink in self.sinks.drain(..) {
      sink.stop();
    }
  }
}

#[derive(Debug)]
pub struct Chatsound {
  url: String,
  data: Option<Vec<u8>>,
}

impl Chatsound {
  pub fn from_url(url: String) -> Self {
    Self { url, data: None }
  }

  pub fn download(&mut self) {
    let mut response = reqwest::get(&self.url).unwrap();

    let mut data = Vec::new();
    response.read_to_end(&mut data).unwrap();

    self.data = Some(data);
  }

  pub fn play(&mut self, device: &Device, sinks: &mut Vec<Sink>) {
    if self.data.is_none() {
      self.download();
    }

    if let Some(data) = &self.data {
      let reader = BufReader::new(Cursor::new(data.clone()));

      let source = Decoder::new(reader).unwrap();

      let sink = Sink::new(&device);
      sink.set_volume(0.1);

      sink.append(source);
      sinks.push(sink);
    }
  }
}

#[test]
fn it_works() {
  /*
  [["melee_impacts","mvm weapon bottle brokenhitflesh","melee_impacts/mvm weapon bottle brokenhitflesh/1.ogg"],["melee_impacts","mvm weapon bottle brokenhitflesh","melee_impacts/mvm weapon bottle brokenhitflesh/2.ogg"],["melee_impacts","mvm weapon bottle brokenhitflesh","melee_impacts/mvm weapon bottle brokenhitflesh/3.ogg"],["melee_impacts","mvm weapon sword hi
  */
  
  let mut chatsounds = Chatsounds::new();
  chatsounds.load_github_msgpack(
    "PAC3-Server/chatsounds-valve-games".to_string(),
    "tf2".to_string(),
  );

  let mut found = chatsounds.find("a");

  println!("{:?}", found);
  // let mut loaded = found[0].download();
  // loaded.play(&chatsounds.sink);

  // std::thread::sleep_ms(2000);
}
