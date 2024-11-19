use crate::error::Result;
use futures_util::TryStreamExt;
use reqwest::IntoUrl;
use serde::Serialize;
use std::{collections::HashMap, path::Path};
use tauri::{Emitter, Runtime, Window};
use tokio::{fs::File, io::AsyncWriteExt};


pub const DOWNLOAD_EVENT: &str = "download://progress";

#[derive(Clone, Serialize)]
struct ProgressPayload {
    id: u32,
    progress: u64,
    total: u64
}

pub async fn download<R: Runtime, U: IntoUrl, P: AsRef<Path>>(
    window: Window<R>,
    id: u32,
    url: U,
    file_path: P,
    headers: HashMap<&str, &str>,
) -> Result<u32>{
    let builder = reqwest::Client::builder();
    let client = builder.https_only(true).build().unwrap();

    let mut request = client.get(url);
    for(key, value) in headers{
        request = request.header(key, value);
    }

    let response = request.send().await?;
    let total = response.content_length().unwrap_or(0);

    let mut file = File::create(file_path).await?;
    let mut stream = response.bytes_stream();

    let mut downloaded_len: u64 = 0;

    while let Some(chunk) = stream.try_next().await?{
        file.write_all(&chunk).await?;

        downloaded_len += chunk.len() as u64;

        let _ = window.emit(DOWNLOAD_EVENT, ProgressPayload{
            id, progress: downloaded_len, total,
        });
    }

    Ok(id)
}
