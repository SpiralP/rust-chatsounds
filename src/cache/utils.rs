use anyhow::Result;
use bytes::Bytes;
use reqwest::Client;

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

pub async fn get<S: AsRef<str>>(url: S) -> Result<(Bytes, Option<String>)> {
    let client = make_client()?;
    let response = client.get(url.as_ref()).send().await?;

    let etag = response
        .headers()
        .get(reqwest::header::ETAG)
        .and_then(|value| value.to_str().ok())
        .map(|value| value.to_string());

    let bytes = response.bytes().await?;

    Ok((bytes, etag))
}

pub async fn head_etag<S: AsRef<str>>(url: S) -> Result<Option<String>> {
    let client = make_client()?;
    let response = client.head(url.as_ref()).send().await?;

    let etag = response
        .headers()
        .get(reqwest::header::ETAG)
        .and_then(|value| value.to_str().ok())
        .map(|value| value.to_string());

    Ok(etag)
}
