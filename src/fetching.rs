use std::path::{Component, Path};

pub use rodio::{queue::SourcesQueueOutput, Decoder, Device, Sample, Sink, Source, SpatialSink};
use serde::Deserialize;

use crate::{cache::download, error::Result, types::Chatsound, Chatsounds, Error};

#[derive(Deserialize)]
pub struct GitHubApiFileEntry {
    pub path: String,
    pub r#type: String,
    pub size: Option<usize>,
}

#[derive(Deserialize)]
pub struct GitHubApiTrees {
    pub tree: Vec<GitHubApiFileEntry>,
}

pub type GitHubMsgpackEntries = Vec<Vec<String>>;

impl Chatsounds {
    pub async fn fetch_github_api(&self, repo: &str, _repo_path: &str) -> Result<GitHubApiTrees> {
        let api_url = format!(
            "https://api.github.com/repos/{}/git/trees/HEAD?recursive=1",
            repo
        );

        #[cfg(feature = "fs")]
        let cache = &self.cache_path;
        #[cfg(feature = "memory")]
        let cache = self.fs_memory.clone();

        let bytes = download(&api_url, cache, true).await?;

        let trees: GitHubApiTrees =
            serde_json::from_slice(&bytes).map_err(|err| Error::Json { err, url: api_url })?;

        Ok(trees)
    }

    pub fn load_github_api(
        &mut self,
        repo: &str,
        repo_path: &str,
        mut trees: GitHubApiTrees,
    ) -> Result<()> {
        for entry in trees.tree.iter_mut() {
            if entry.r#type != "blob" || !entry.path.starts_with(repo_path) {
                continue;
            }

            // e26/stop.ogg or e26/nestetrismusic/1.ogg
            let sound_path = entry.path.split_off(repo_path.len() + 1);

            let path = Path::new(&sound_path);
            if let Some(Component::Normal(s)) = path.components().nth(1) {
                if let Some(sentence) = Path::new(&s).file_stem() {
                    let sentence = sentence.to_string_lossy().to_string();
                    let vec = self.map_store.entry(sentence).or_default();
                    let chatsound = Chatsound {
                        repo: repo.to_string(),
                        repo_path: repo_path.to_string(),
                        sound_path,
                    };

                    let url = chatsound.get_url();
                    match vec.binary_search_by(|c| c.get_url().cmp(&url)) {
                        Ok(_pos) => {
                            // already exists, don't add again
                        }
                        Err(pos) => {
                            vec.insert(pos, chatsound);
                        }
                    }
                }
            }
        }

        Ok(())
    }

    pub async fn fetch_github_msgpack(
        &self,
        repo: &str,
        repo_path: &str,
    ) -> Result<GitHubMsgpackEntries> {
        let msgpack_url = format!(
            "https://raw.githubusercontent.com/{}/HEAD/{}/list.msgpack",
            repo, repo_path
        );

        #[cfg(feature = "fs")]
        let cache = &self.cache_path;
        #[cfg(feature = "memory")]
        let cache = self.fs_memory.clone();

        let bytes = download(&msgpack_url, cache, true).await?;
        let entries: GitHubMsgpackEntries =
            rmp_serde::decode::from_slice(&bytes).map_err(|err| Error::Msgpack {
                err,
                url: msgpack_url,
            })?;

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
            let vec = self.map_store.entry(sentence.to_string()).or_default();

            let chatsound = Chatsound {
                repo: repo.to_string(),
                repo_path: repo_path.to_string(),
                sound_path,
            };

            let url = chatsound.get_url();
            match vec.binary_search_by(|c| c.get_url().cmp(&url)) {
                Ok(_pos) => {
                    // already exists, don't add again
                }
                Err(pos) => {
                    vec.insert(pos, chatsound);
                }
            }
        }

        Ok(())
    }
}
