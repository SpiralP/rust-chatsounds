use bytes::Bytes;
use quick_error::ResultExt;
use reqwest::{
    header::{self, HeaderValue},
    Client, StatusCode,
};

use crate::error::{Error, Result};

static APP_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));

fn make_client() -> reqwest::Result<Client> {
    #[cfg(not(feature = "wasm"))]
    {
        Client::builder().user_agent(APP_USER_AGENT).build()
    }

    #[cfg(feature = "wasm")]
    {
        Client::builder().build()
    }
}

/// * changed content or non-etag call -> Some(response_bytes, new_etag)
/// * content not changed -> None
pub async fn get_with_etag(
    url: &str,
    last_etag: HeaderValue,
) -> Result<Option<(Bytes, Option<HeaderValue>)>> {
    let client = make_client().map_err(Error::ReqwestMakeClient)?;
    let response = client
        .get(url)
        .header(header::IF_NONE_MATCH, last_etag)
        .send()
        .await
        .context(url)?;
    let etag = response.headers().get(header::ETAG).cloned();

    let status = response.status();
    if status.is_success() {
        let bytes = response.bytes().await.context(url)?;

        Ok(Some((bytes, etag)))
    } else if status == StatusCode::NOT_MODIFIED {
        Ok(None)
    } else {
        // TODO response text if bad
        Err(Error::ReqwestStatus(status, url.to_owned()))
    }
}

pub async fn get(url: &str) -> Result<(Bytes, Option<HeaderValue>)> {
    let client = make_client().map_err(Error::ReqwestMakeClient)?;
    let response = client.get(url).send().await.context(url)?;
    let etag = response.headers().get(header::ETAG).cloned();

    let status = response.status();
    if status.is_success() {
        let bytes = response.bytes().await.context(url)?;

        Ok((bytes, etag))
    } else {
        // TODO response text if bad
        Err(Error::ReqwestStatus(status, url.to_owned()))
    }
}

#[cfg(test)]
#[tokio::test]
async fn test_get_not_changed() {
    // got 304 without passing etag (ie we don't have anything cached)
    let result = get("https://httpbin.org/status/304").await;
    assert!(result.is_err());

    // request new content and obtain new etag
    let (_bytes, etag) = get("https://httpbin.org/etag/hello").await.unwrap();
    assert_eq!(etag.unwrap().to_str().unwrap(), "hello");

    // content not changed (using etag)
    let res = get_with_etag(
        "https://httpbin.org/etag/hello",
        HeaderValue::from_static("hello"),
    )
    .await
    .unwrap();
    assert_eq!(res, None);

    // content changed (using old etag)
    let (_bytes, etag) = get_with_etag(
        "https://httpbin.org/etag/meow",
        HeaderValue::from_static("hello"),
    )
    .await
    .unwrap()
    .unwrap();
    assert_eq!(etag.unwrap().to_str().unwrap(), "meow");
}
