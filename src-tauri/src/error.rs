use log::SetLoggerError;
use serde::{ser::Serializer, Serialize};
use std::num::ParseIntError;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error{
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Request(#[from] reqwest::Error),
    #[error(transparent)]
    IntParse(#[from] ParseIntError),
    #[error(transparent)]
    Regex(#[from] regex::Error),
    #[error(transparent)]
    URLParse(#[from] url::ParseError),
    #[error(transparent)]
    Logger(#[from] SetLoggerError),
    #[error(transparent)]
    Tauri(#[from] tauri::Error),
    #[error("{0}")]
    Other(String),
}


impl Serialize for Error{
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,{
            serializer.serialize_str(self.to_string().as_ref())
        }
}