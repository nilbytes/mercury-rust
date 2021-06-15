use http;
use serde_json;
use serde_qs;
use reqwest;
use std::io;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("{0}")]
    Http(#[from] reqwest::Error),
    #[error("{0}")]
    Json(#[from] serde_json::error::Error),
    #[error("{0}")]
    Uri(#[from] http::uri::InvalidUri),
    #[error("{0}")]
    Io(#[from] io::Error),
    #[error("{0}")]
    QsError(#[from] serde_qs::Error),
    #[error("{0}")]
    HeaderError(#[from] reqwest::header::InvalidHeaderValue),
    #[error("Response error {:?}", .errors)]
    MercuryError { errors: ErrorMessage },
    #[error("Other/Unknown")]
    Unknown,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ErrorResponse {
    pub errors: ErrorMessage
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ErrorMessage {
    pub message: String
}