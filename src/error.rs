use std::fmt::{self, Display, Formatter};

#[derive(thiserror::Error, Debug, PartialEq)]
pub enum AppError {
    NotFound,
    URLParseError,
}

impl Display for AppError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            AppError::NotFound => write!(f, "Not found"),
            AppError::URLParseError => write!(f, "URL parse error"),
        }
    }
}
