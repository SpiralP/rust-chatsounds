use std::path::{Path, PathBuf};

use crate::types::Chatsound;

quick_error! {
    #[derive(Debug)]
    pub enum Error {
        DirMissing(path: PathBuf) {
            display("{:?} is not a directory", path)
        }

        EmptyChoose(text: String) {
            display("empty choose for {:?}", text)
        }

        GitHub(err: String, url: String) {
            display("GitHub Error for {:?}: {:?}", url, err)
        }

        Io(err: std::io::Error, path: PathBuf) {
            context(path: AsRef<Path>, err: std::io::Error)
                -> (err, path.as_ref().to_owned())
        }

        RodioPlayError(err: rodio::PlayError) { from() }

        RodioStreamError(err: rodio::StreamError) { from() }

        RodioDecoderError(err: rodio::decoder::DecoderError, sound_path: String) {
            context(chatsound: &Chatsound, err: rodio::decoder::DecoderError)
                -> (err, chatsound.sound_path.to_owned())
        }

        MsgpackError(err: rmp_serde::decode::Error, url:String) {
            context(url: AsRef<str>, err: rmp_serde::decode::Error)
                -> (err, url.as_ref().to_owned())
        }

        JsonError(err: serde_json::Error, url: String) {
            context(url: AsRef<str>, err: serde_json::Error)
                -> (err, url.as_ref().to_owned())
        }

        ReqwestMakeClient(err: reqwest::Error) { }
        ReqwestWithContext(err: reqwest::Error, url: String) {
            context(url: AsRef<str>, err: reqwest::Error)
                -> (err, url.as_ref().to_owned())
        }

        ReqwestStatus(status: reqwest::StatusCode, url: String) {
            display("reqwest http status Error: {:?} {:?}", status, url)
        }

        Nom(err: String, text: String) {
            display("nom Error for {:?}: {:?}", text, err)
        }
    }
}

pub type Result<T> = std::result::Result<T, Error>;

#[macro_export]
macro_rules! ensure {
    ($test:expr, $err:expr) => {
        if !$test {
            return Err($err);
        }
    };
}

#[macro_export]
macro_rules! bail {
    ($err:expr) => {
        return Err($err);
    };
}
