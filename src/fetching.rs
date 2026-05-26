use std::path::{Component, Path};

use serde::Deserialize;

use crate::{
    Chatsounds, Error, cache::download, error::Result, parsing::normalize_sentence,
    types::Chatsound,
};

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

/// Extract `(sentence_key, sound_path)` from a GitHub-API tree entry path,
/// returning `None` for entries that should not become a `map_store` key:
/// outside `repo_path`, top-level files (no category dir), dotfiles like
/// `.gitkeep`, or names that normalize to an empty key.
///
/// Mirrors the existing `<repo_path>/<category>/<file>` schema — the
/// second component of the relative path is what becomes the key. For
/// nested files (`<cat>/<subcat>/<file>`) the subcat name is the key,
/// which is how multiple takes of the same sound share a lookup key.
fn parse_api_entry(repo_path: &str, entry_path: &str) -> Option<(String, String)> {
    let suffix = entry_path
        .strip_prefix(repo_path)
        .and_then(|s| s.strip_prefix('/'))?;
    let path = Path::new(suffix);
    let Component::Normal(filename) = path.components().nth(1)? else {
        return None;
    };
    if filename.to_string_lossy().starts_with('.') {
        return None;
    }
    let stem = Path::new(filename).file_stem()?;
    let key = normalize_sentence(&stem.to_string_lossy());
    if key.is_empty() {
        return None;
    }
    Some((key, suffix.to_string()))
}

impl Chatsounds {
    pub async fn fetch_github_api(&self, repo: &str, _repo_path: &str) -> Result<GitHubApiTrees> {
        let api_url = format!("https://api.github.com/repos/{repo}/git/trees/HEAD?recursive=1");

        #[cfg(feature = "fs")]
        let cache = &self.cache_path;
        #[cfg(feature = "memory")]
        let cache = self.fs_memory.clone();

        let bytes = download(&api_url, cache, false).await?;

        let trees: GitHubApiTrees =
            serde_json::from_slice(&bytes).map_err(|err| Error::Json { err, url: api_url })?;

        Ok(trees)
    }

    pub fn load_github_api(
        &mut self,
        repo: &str,
        repo_path: &str,
        trees: GitHubApiTrees,
    ) -> Result<()> {
        for entry in trees.tree {
            if entry.r#type != "blob" {
                continue;
            }
            let Some((sentence, sound_path)) = parse_api_entry(repo_path, &entry.path) else {
                continue;
            };

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

        Ok(())
    }

    pub async fn fetch_github_msgpack(
        &self,
        repo: &str,
        repo_path: &str,
    ) -> Result<GitHubMsgpackEntries> {
        let msgpack_url =
            format!("https://raw.githubusercontent.com/{repo}/HEAD/{repo_path}/list.msgpack");

        #[cfg(feature = "fs")]
        let cache = &self.cache_path;
        #[cfg(feature = "memory")]
        let cache = self.fs_memory.clone();

        let bytes = download(&msgpack_url, cache, false).await?;
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
        entries: GitHubMsgpackEntries,
    ) -> Result<()> {
        for entry in entries {
            // e26/stop.ogg or e26/nestetrismusic/1.ogg
            let sentence = normalize_sentence(&entry[1]);
            if sentence.is_empty() {
                continue;
            }
            let sound_path = entry[2].clone();
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

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::parse_api_entry;

    #[test]
    fn weird_filenames_from_user() {
        let repo = "sounds";

        assert_eq!(
            parse_api_entry(repo, "sounds/sammich/yes no yes no YES NO.ogg"),
            Some((
                "yes no yes no YES NO".to_string(),
                "sammich/yes no yes no YES NO.ogg".to_string(),
            )),
        );
        assert_eq!(
            parse_api_entry(
                repo,
                "sounds/bill_wurtz/hell yeah now we've got business.ogg"
            ),
            Some((
                "hell yeah now weve got business".to_string(),
                "bill_wurtz/hell yeah now we've got business.ogg".to_string(),
            )),
        );
        assert_eq!(
            parse_api_entry(repo, "sounds/piano ogg/0-a.ogg"),
            Some(("0 a".to_string(), "piano ogg/0-a.ogg".to_string())),
        );
        assert_eq!(
            parse_api_entry(repo, "sounds/test/ file test  .ogg"),
            Some(("file test".to_string(), "test/ file test  .ogg".to_string())),
        );
        assert_eq!(
            parse_api_entry(repo, "sounds/test/!file_test!.ogg"),
            Some(("file test".to_string(), "test/!file_test!.ogg".to_string())),
        );
    }

    #[test]
    fn dotfiles_are_skipped() {
        let repo = "sounds";
        assert_eq!(parse_api_entry(repo, "sounds/sammich/.gitkeep"), None);
        assert_eq!(parse_api_entry(repo, "sounds/cat/.DS_Store"), None);
        assert_eq!(parse_api_entry(repo, "sounds/cat/.gitignore"), None);
        // Even a `.hidden.ogg` is a dotfile per Unix conventions.
        assert_eq!(parse_api_entry(repo, "sounds/cat/.hidden.ogg"), None);
    }

    #[test]
    fn top_level_files_are_skipped() {
        // The existing schema expects `<repo_path>/<category>/<file>`; files
        // directly under repo_path (no category) are dropped.
        let repo = "sounds";
        assert_eq!(parse_api_entry(repo, "sounds/README.md"), None);
        assert_eq!(parse_api_entry(repo, "sounds/file.ogg"), None);
    }

    #[test]
    fn outside_repo_path_is_skipped() {
        let repo = "sounds";
        assert_eq!(parse_api_entry(repo, "other/cat/file.ogg"), None);
        // Prefix match without the trailing slash boundary must not pass.
        assert_eq!(parse_api_entry(repo, "soundsthing/cat/file.ogg"), None);
        assert_eq!(parse_api_entry(repo, "sounds"), None);
        assert_eq!(parse_api_entry(repo, ""), None);
    }

    #[test]
    fn empty_after_normalization_is_skipped() {
        let repo = "sounds";
        assert_eq!(parse_api_entry(repo, "sounds/cat/!!!.ogg"), None);
        assert_eq!(parse_api_entry(repo, "sounds/cat/'''.ogg"), None);
        assert_eq!(parse_api_entry(repo, "sounds/cat/   .ogg"), None);
    }

    #[test]
    fn subdir_takes_subdir_name_as_key() {
        // Existing behavior — `<cat>/<subcat>/<file>` keys by subcat, so all
        // `1.ogg`, `2.ogg`... in `e26/nestetrismusic/` share key
        // `nestetrismusic`.
        let repo = "sounds";
        assert_eq!(
            parse_api_entry(repo, "sounds/e26/nestetrismusic/1.ogg"),
            Some((
                "nestetrismusic".to_string(),
                "e26/nestetrismusic/1.ogg".to_string(),
            )),
        );
    }

    #[test]
    fn category_with_punctuation_still_works() {
        // The category dir name lands in `sound_path`, not the key; the key
        // only cares about the file leaf.
        let repo = "sounds";
        assert_eq!(
            parse_api_entry(repo, "sounds/!weird cat!/hello.ogg"),
            Some(("hello".to_string(), "!weird cat!/hello.ogg".to_string())),
        );
    }

    #[test]
    fn file_with_no_extension_keeps_whole_name() {
        let repo = "sounds";
        assert_eq!(
            parse_api_entry(repo, "sounds/cat/foo"),
            Some(("foo".to_string(), "cat/foo".to_string())),
        );
    }
}
