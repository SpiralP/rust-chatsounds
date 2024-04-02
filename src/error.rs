use std::path::PathBuf;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("DirMissing: {path}")]
    DirMissing { path: PathBuf },

    #[error("EmptyChoose: {text}")]
    EmptyChoose { text: String },

    #[error("GitHub: {err}: {url}")]
    GitHub { err: String, url: String },

    #[error("Io: {err}: {path}")]
    Io {
        #[source]
        err: std::io::Error,
        path: PathBuf,
    },

    #[error("RodioPlayError: {err}")]
    RodioPlay {
        #[from]
        err: rodio::PlayError,
    },

    #[error("RodioStreamError: {err}")]
    RodioStream {
        #[from]
        err: rodio::StreamError,
    },

    #[error("RodioDecoder: {err}: {sound_path}")]
    RodioDecoder {
        #[source]
        err: rodio::decoder::DecoderError,
        sound_path: String,
    },

    #[error("Msgpack: {err}: {url}")]
    Msgpack {
        #[source]
        err: rmp_serde::decode::Error,
        url: String,
    },

    #[error("Json: {err}: {url}")]
    Json {
        #[source]
        err: serde_json::Error,
        url: String,
    },

    #[error("ReqwestMakeClientError: {err}")]
    ReqwestMakeClient {
        #[source]
        err: reqwest::Error,
    },

    #[error("Reqwest: {err}: {url}")]
    Reqwest {
        #[source]
        err: reqwest::Error,
        url: String,
    },

    #[error("ReqwestStatus: {status}: {url}")]
    ReqwestStatus {
        status: reqwest::StatusCode,
        url: String,
    },

    #[error("InvalidHeaderValue: {err}: {url}")]
    InvalidHeaderValue {
        #[source]
        err: reqwest::header::InvalidHeaderValue,
        url: String,
    },

    #[error("Nom: {err}: {text}")]
    Nom {
        #[source]
        err: nom::Err<nom::error::Error<String>>,
        text: String,
    },
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

#[test]
fn test_error_nom() {
    //
    todo!();
}
