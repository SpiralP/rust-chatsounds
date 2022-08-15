use bytes::Bytes;
use quick_error::ResultExt;
use reqwest::Client;

use crate::{
    ensure,
    error::{Error, Result},
};

static APP_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));

pub fn make_client() -> reqwest::Result<Client> {
    #[cfg(not(feature = "wasm"))]
    {
        Client::builder().user_agent(APP_USER_AGENT).build()
    }

    #[cfg(feature = "wasm")]
    {
        Client::builder().build()
    }
}

pub async fn get(url: &str) -> Result<(Bytes, Option<String>)> {
    let client = make_client().map_err(Error::ReqwestMakeClient)?;
    let response = client.get(url).send().await.context(url)?;

    let status = response.status();
    ensure!(
        status.is_success(),
        Error::ReqwestStatus(status, url.to_owned())
    );

    let etag = response
        .headers()
        .get(reqwest::header::ETAG)
        .and_then(|value| value.to_str().ok())
        .map(|value| value.to_string());

    let bytes = response.bytes().await.context(url)?;

    Ok((bytes, etag))
}

pub async fn head_etag(url: &str) -> Result<Option<String>> {
    let client = make_client().map_err(Error::ReqwestMakeClient)?;
    let response = client.head(url).send().await.context(url)?;

    let etag = response
        .headers()
        .get(reqwest::header::ETAG)
        .and_then(|value| value.to_str().ok())
        .map(|value| value.to_string());

    Ok(etag)
}
