mod format;

use crate::turing::error::NotFoundError;

pub use format::AliasFormatError;

#[derive(Debug)]
pub enum StateError {
    Format(AliasFormatError),
    NotFound(NotFoundError<String>),
}

impl From<AliasFormatError> for StateError {
    fn from(value: AliasFormatError) -> Self {
        Self::Format(value)
    }
}

impl From<NotFoundError<String>> for StateError {
    fn from(value: NotFoundError<String>) -> Self {
        Self::NotFound(value)
    }
}

impl std::fmt::Display for StateError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use StateError::*;
        write!(
            f,
            "{}",
            match self {
                Format(err) => err.to_string(),
                NotFound(err) => err.to_string(),
            }
        )
    }
}

impl std::error::Error for StateError {}
