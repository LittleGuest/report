use thiserror::Error;

pub mod http;
pub mod report;
pub mod schema;

#[derive(Debug, Error)]
pub enum ReportError {
    #[error("invalid header {0}")]
    InvalidHeader(String),
    #[error("header name error")]
    ReqwestHeaderNameError(#[from] reqwest::header::InvalidHeaderName),
    #[error("header value error")]
    ReqwestHeaderValueError(#[from] reqwest::header::InvalidHeaderValue),
    #[error("unknown error")]
    Unknown,
}

pub type ReportResult<T> = anyhow::Result<T, ReportError>;
