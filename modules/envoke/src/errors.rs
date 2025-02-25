use std::error::Error as StdError;

use thiserror::Error;

#[doc(hidden)]
pub type Result<T> = std::result::Result<T, Error>;

#[doc(hidden)]
pub type BoxError = Box<dyn StdError + Send + Sync + 'static>;

#[derive(Debug, Error, strum::EnumIs)]
pub enum ParseError {
    #[error("key-value pair has no key")]
    MissingKey,

    #[error("key-value pair has no value")]
    MissingValue,

    #[error("found equalsign with no key or value around it")]
    UnexpectedEqualsign,

    #[error("key `{key}` is of unexpected type")]
    UnexpectedKeyType { key: String },

    #[error("value `{value}` is of unexpected type")]
    UnexpectedValueType { value: String },
}

#[derive(Debug, Error, strum::EnumIs)]
pub enum RetrieveError {
    #[error("none of the environment variables ({keys}) was found")]
    NotFound { keys: String },

    #[error("environment variable `{key}` contains invalid Unicode")]
    InvalidUnicode { key: String },

    #[error("fatal error occurred")]
    Fatal,
}

#[derive(Debug, Error, strum::EnumIs)]
pub enum Error {
    #[error("Retrieve error occurred: {0}")]
    RetrieveError(#[from] RetrieveError),

    #[error("Parse error occurred: {0}")]
    ParseError(#[from] ParseError),

    #[error("Failed to convert environment variable `{key}` to expected type")]
    ConvertError { key: String },

    #[error("Validation error occurred for `{field}`: {err}")]
    ValidationError {
        field: String,
        #[source]
        err: BoxError,
    },
}
