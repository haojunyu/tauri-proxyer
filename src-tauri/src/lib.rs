// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#![cfg_attr(not(debug_assertions), windows_subsystem = "macos")]

mod error;
mod util;

#[macro_use]
extern crate log;

use error::Result;
use lazy_static::lazy_static;
use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};
use std::collections::HashMap;
use std::path::PathBuf;
use url::Url;
use util::download::download;



const APP_NAME: &str = "tauri-proxyer";
const NON_ALPHANUMERIC: &AsciiSet = &CONTROLS.add(b'/').add(b':');

lazy_static! {
    pub static ref EXECUTABLE_DIR: PathBuf = dirs::cache_dir().unwrap().join(APP_NAME);
}

#[tauri::command]
async fn download_executable_file(window: tauri::Window, id:u32) -> Result<PathBuf>{
    let mut cdn = Url::parse("https://ghproxy.com/")?;

    #[cfg(all(target_os="windows", target_arch = "x86_64"))]
    let url = "https://github.com/p4gefau1t/trojan-go/releases/download/v0.10.6/trojan-go-windows-amd64.zip";

    #[cfg(all(target_os="macos", target_arch = "x86_64"))]
    let url = "https://github.com/p4gefau1t/trojan-go/releases/download/v0.10.6/trojan-go-darwin-amd64.zip";

    #[cfg(all(target_os="macos", target_arch = "aarch64"))]
    let url = "https://github.com/p4gefau1t/trojan-go/releases/download/v0.10.6/trojan-go-darwin-arm64.zip";

    cdn.set_path(&utf8_percent_encode(url, NON_ALPHANUMERIC).to_string());

    let file_path = EXECUTABLE_DIR.join("trojan-go.zip");

    match download(
        window,
        id, 
        cdn, 
        &file_path, 
        HashMap::from([("user-Agent", 
            "Mozilla/5.0(Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/114.0.0.0 Safari/537.36 Edg/114.0.1823.82")]),
    ).await{
        Ok(_) => Ok(file_path.to_owned()),
        Err(e) => {
            error!("下载文件时出错: {}", e);
            Err(e)
        }
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![download_executable_file])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
