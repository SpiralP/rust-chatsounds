use bytes::Bytes;
use reqwest::{
    Client, StatusCode,
    header::{self, HeaderValue},
};

use crate::error::{Error, Result};

static APP_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));

#[cfg(not(feature = "wasm"))]
static GITHUB_TOKEN: std::sync::LazyLock<Option<String>> = std::sync::LazyLock::new(|| {
    std::env::var("GITHUB_TOKEN")
        .ok()
        .map(|s| s.trim().to_owned())
        .filter(|s| !s.is_empty())
});

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

#[cfg(not(feature = "wasm"))]
fn github_auth_header(url: &str) -> Option<HeaderValue> {
    let token = GITHUB_TOKEN.as_deref()?;
    let host = reqwest::Url::parse(url)
        .ok()?
        .host_str()?
        .to_ascii_lowercase();
    if !matches!(
        host.as_str(),
        "api.github.com" | "raw.githubusercontent.com" | "codeload.github.com"
    ) {
        return None;
    }
    let mut hv = HeaderValue::from_str(&format!("Bearer {token}")).ok()?;
    hv.set_sensitive(true);
    Some(hv)
}

#[cfg(feature = "wasm")]
fn github_auth_header(_url: &str) -> Option<HeaderValue> {
    None
}

/// * changed content or non-etag call -> `Some(response_bytes, new_etag)`
/// * content not changed -> None
pub async fn get_with_etag(
    url: &str,
    last_etag: HeaderValue,
) -> Result<Option<(Bytes, Option<HeaderValue>)>> {
    let client = make_client().map_err(|err| Error::ReqwestMakeClient { err })?;
    let mut req = client.get(url).header(header::IF_NONE_MATCH, last_etag);
    if let Some(auth) = github_auth_header(url) {
        req = req.header(header::AUTHORIZATION, auth);
    }
    let response = req.send().await.map_err(|err| Error::Reqwest {
        err,
        url: url.into(),
    })?;
    let etag = response.headers().get(header::ETAG).cloned();

    let status = response.status();
    if status.is_success() {
        let bytes = response.bytes().await.map_err(|err| Error::Reqwest {
            err,
            url: url.into(),
        })?;

        Ok(Some((bytes, etag)))
    } else if status == StatusCode::NOT_MODIFIED {
        Ok(None)
    } else {
        // TODO response text if bad
        Err(Error::ReqwestStatus {
            status,
            url: url.into(),
        })
    }
}

pub async fn get(url: &str) -> Result<(Bytes, Option<HeaderValue>)> {
    let client = make_client().map_err(|err| Error::ReqwestMakeClient { err })?;
    let mut req = client.get(url);
    if let Some(auth) = github_auth_header(url) {
        req = req.header(header::AUTHORIZATION, auth);
    }
    let response = req.send().await.map_err(|err| Error::Reqwest {
        err,
        url: url.into(),
    })?;
    let etag = response.headers().get(header::ETAG).cloned();

    let status = response.status();
    if status.is_success() {
        let bytes = response.bytes().await.map_err(|err| Error::Reqwest {
            err,
            url: url.into(),
        })?;

        Ok((bytes, etag))
    } else {
        // TODO response text if bad
        Err(Error::ReqwestStatus {
            status,
            url: url.into(),
        })
    }
}

#[cfg(test)]
#[tokio::test]
#[ignore]
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
