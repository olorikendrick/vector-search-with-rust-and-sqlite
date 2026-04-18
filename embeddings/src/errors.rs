use std::{fmt::Display, io};
#[derive(Debug)]
pub enum AppError {
    IOError(std::io::Error),
    NetworkError(reqwest::Error),
    DBError(rusqlite::Error),
    EnvError(std::env::VarError),
    ParseError(String),
     DimensionMismatch { expected: usize, got: usize },
     ZeroVector
     
    

}

impl Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AppError::IOError(e) => match e.kind() {
                io::ErrorKind::NotFound => write!(f, "File not found"),
                io::ErrorKind::PermissionDenied => write!(f, "Permission denied"),
                _ => write!(f, "IO error: {e}"),
            },
            AppError::NetworkError(e) => match e.status() {
                Some(status) => match status {
                    reqwest::StatusCode::UNAUTHORIZED => write!(f, "Invalid API key"),
                    reqwest::StatusCode::TOO_MANY_REQUESTS => write!(f, "Rate limit exceeded"),
                    reqwest::StatusCode::NOT_FOUND => write!(f, "API endpoint not found"),
                    reqwest::StatusCode::INTERNAL_SERVER_ERROR => write!(f, "Gemini API error"),
                    _ => write!(f, "Network error: {e}"),
                },
                None => {
                    if e.is_timeout() {
                        write!(f, "Request timed out")
                    } else if e.is_connect() {
                        write!(f, "Connection failed")
                    } else {
                        write!(f, "Network error: {e}")
                    }
                }
            },
            AppError::DBError(e) => write!(f, "Database error: {e}"),
            AppError::EnvError(e) => write!(f, "Missing environment variable: {e}"),
            AppError::ParseError(e) => write!(f, "Parse error: {e}"),
            AppError::DimensionMismatch { expected, got } => {
    write!(f, "Vector dimension mismatch: expected {expected}, got {got}")
},
AppError::ZeroVector => write!(f, "Cannot compute similarity for a zero vector"),
        }
    }
}

impl std::error::Error for AppError {}
macro_rules! impl_from {
    ($variant:ident, $error:ty) => {
        impl From<$error> for AppError {
            fn from(e: $error) -> Self {
                Self::$variant(e)
            }
        }
    };
}

impl_from!(IOError, io::Error);
impl_from!(NetworkError, reqwest::Error);
impl_from!(DBError, rusqlite::Error);
impl_from!(EnvError, std::env::VarError);
impl_from!(ParseError, String);
