use rodio::{Decoder, Device, Sink, Source};
use serde::Deserialize;
use std::{
  collections::HashMap,
  io::{prelude::*, BufReader, Cursor},
  path::{Component, Path, PathBuf},
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
  pub sink: Sink,
  stores: Vec<ChatsoundsStore>,
}

impl Chatsounds {
  /// will download list of files from github `repo`, filtering to `repo_path`
  pub fn new(repo_infos: Vec<(String, String)>) -> Self {
    let mut stores = Vec::new();

    for repo_info in repo_infos {
      let repo = repo_info.0;
      let repo_path = repo_info.1;

      let api_url = format!(
        "https://api.github.com/repos/{}/git/trees/master?recursive=1",
        repo
      );

      let mut response = reqwest::get(&api_url).unwrap();
      let mut trees: GitHubApiTrees = response.json().unwrap();

      let mut store: HashMap<String, Vec<String>> = HashMap::new();

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

      stores.push(ChatsoundsStore {
        repo,
        repo_path,
        store,
      });
    }

    let device = rodio::default_output_device().unwrap();
    let sink = Sink::new(&device);

    Self { sink, stores }
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
}

#[derive(Debug)]
pub struct Chatsound {
  url: String,
}

impl Chatsound {
  pub fn from_url(url: String) -> Self {
    Self { url }
  }

  pub fn download(&mut self) -> LoadedChatsound {
    let mut response = reqwest::get(&self.url).unwrap();

    let mut data = Vec::new();
    response.read_to_end(&mut data).unwrap();

    LoadedChatsound { data }
  }
}

pub struct LoadedChatsound {
  data: Vec<u8>,
}

impl LoadedChatsound {
  pub fn play(&mut self, sink: &Sink) {
    // TODO don't clone?
    let reader = BufReader::new(Cursor::new(self.data.clone()));

    let source = Decoder::new(reader).unwrap();
    sink.append(source);
    // rodio::play_raw(&device, source.convert_samples());
  }
}

#[test]
fn it_works() {
  let mut chatsounds = Chatsounds::new(vec![
    (
      "Metastruct/garrysmod-chatsounds".to_string(),
      "sound/chatsounds/autoadd".to_string(),
    ),
    (
      "PAC3-Server/chatsounds".to_string(),
      "sounds/chatsounds".to_string(),
    ),
  ]);

  let mut found = chatsounds.find("a");

  println!("{:?}", found);
  // let mut loaded = found[0].download();
  // loaded.play(&chatsounds.sink);

  std::thread::sleep_ms(2000);
}
