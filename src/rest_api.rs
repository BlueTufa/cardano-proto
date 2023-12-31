use reqwest::header::HeaderMap;
use reqwest::Error;
use serde::de::DeserializeOwned;
use std::fs::File;
use std::io::{copy, Cursor};
use std::path::PathBuf;

/// A Rest API helper which simplifies the deserialization from JSON.
/// Optionally support HTTP headers.
/// * `url` - A borrowed URL string which identifies a target endpoint
/// * `headers` - An optional map of HTTP headers for the target endpoint
pub async fn get<T>(url: &str, headers: Option<HeaderMap>) -> Result<T, Error>
where
    T: DeserializeOwned,
{
    let client = if let Some(h) = headers {
        reqwest::Client::new().get(url).headers(h)
    } else {
        reqwest::Client::new().get(url)
    };
    let resp = client.send().await?;
    if !resp.status().is_success() {
        log::warn!("Url {} returned HTTP status: {}", url, resp.status());
        return Err(resp.error_for_status_ref().err().unwrap());
    }
    Ok(resp.json::<T>().await?)
}

pub async fn get_bytes_to_file(url: &str, output_path: PathBuf) -> Result<(), Error> {
    // skip any pre-existing files to meet the idempotency requirement
    if output_path.is_file() {
        return Ok(());
    }
    let resp = reqwest::Client::new().get(url).send().await?;
    if !resp.status().is_success() {
        log::warn!("Url {} returned HTTP status: {}", url, resp.status());
        return Err(resp.error_for_status_ref().err().unwrap());
    }
    let mut content = Cursor::new(resp.bytes().await?);
    let mut dest = { File::create(output_path).expect("File should not exist") };
    copy(&mut content, &mut dest).expect("Unable to write to file");
    Ok(())
}
