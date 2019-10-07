use rodio::{Decoder, Device, Sink};
use serde::Deserialize;
use sha2::{Digest, Sha256};
use std::{
  collections::HashMap,
  fs,
  fs::File,
  io,
  io::{prelude::*, BufReader, Cursor},
  path::{Component, Path, PathBuf},
};

fn cache_download<S: AsRef<str>, P: AsRef<Path>>(url: S, dir_path: P) -> BufReader<File> {
  let mut hasher = Sha256::new();
  hasher.input(url.as_ref());

  let hex_filename = format!("{:x}", hasher.result());

  let path = dir_path.as_ref().join(hex_filename);
  if !fs::metadata(&path)
    .map(|meta| meta.is_file())
    .unwrap_or(false)
  {
    // TODO it's a directory??
    let mut file = File::create(&path).unwrap();
    let mut response = reqwest::get(url.as_ref()).unwrap();
    io::copy(&mut response, &mut file).unwrap();
  }

  BufReader::new(File::open(&path).unwrap())
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

struct ChatsoundsStore {
  repo: String,
  repo_path: String,
  /// [sound_name]: sound_path[]
  store: HashMap<String, Vec<String>>,
}

pub struct Chatsounds {
  cache_path: PathBuf,
  stores: Vec<ChatsoundsStore>,

  pub device: Device,
  pub sinks: Vec<Sink>,
  max_sinks: usize,
}

impl Chatsounds {
  /// will download list of files from github `repo`, filtering to `repo_path`
  pub fn new<T: AsRef<Path>>(cache_path: T) -> Self {
    Self {
      cache_path: cache_path.as_ref().canonicalize().unwrap(),
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

    let response = cache_download(msgpack_url, &self.cache_path);
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

          found.push(Chatsound::from_url(download_url, self.cache_path.clone()))
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
  cache_path: PathBuf,
  data: Option<Vec<u8>>,
}

impl Chatsound {
  pub fn from_url(url: String, cache_path: PathBuf) -> Self {
    Self {
      url,
      data: None,
      cache_path,
    }
  }

  pub fn download(&mut self) {
    let mut response = cache_download(&self.url, &self.cache_path);

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
  
  fs::create_dir_all("cache").unwrap();

  let mut chatsounds = Chatsounds::new("cache");

  println!("Metastruct/garrysmod-chatsounds");
  chatsounds.load_github_api(
    "Metastruct/garrysmod-chatsounds".to_string(),
    "sound/chatsounds/autoadd".to_string(),
  );

  println!("PAC3-Server/chatsounds");
  chatsounds.load_github_api(
    "PAC3-Server/chatsounds".to_string(),
    "sounds/chatsounds".to_string(),
  );

  for folder in &[
    "csgo", "css", "ep1", "ep2", "hl2", "l4d", "l4d2", "portal", "tf2",
  ] {
    println!("PAC3-Server/chatsounds-valve-games {}", folder);
    chatsounds.load_github_msgpack(
      "PAC3-Server/chatsounds-valve-games".to_string(),
      folder.to_string(),
    );
  }

  let found = chatsounds.find("a");

  println!("{:?}", found);
  // let mut loaded = found[0].download();
  // loaded.play(&chatsounds.sink);

  // std::thread::sleep_ms(2000);
}
