use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error, strum::EnumIs)]
pub enum ParseError {
    /// Occurs if the key cannot be found when parsing a key-value pair
    #[error("key-value pair has no key")]
    MissingKey,

    /// Occurs if the value cannot be found when parsing a key-value pair
    #[error("key-value pair has no value")]
    MissingValue,

    /// Occurs when there are no key nor value around the key value delimiter
    #[error("found equalsign with no key or value around it")]
    UnexpectedEqualsign,

    /// Occurs when the key cannot be parsed into the expected type
    #[error("key `{key}` is of unexpected type")]
    UnexpectedKeyType { key: String },

    /// Occurs when the value cannot be parsed into the expected type
    #[error("value `{value}` is of unexpected type")]
    UnexpectedValueType { value: String },
}

#[derive(Debug, Error, strum::EnumIs)]
pub enum RetrieveError {
    /// Occurs when the none of the provided environment variables is found in
    /// the processes's environment
    #[error("none of the environment variables ({keys}) was found")]
    NotFound { keys: String },

    /// Occurs when the provided environment variable name is not valid unicode
    #[error("environment variable `{key}` contains invalid Unicode")]
    InvalidUnicode { key: String },

    /// A situation has occured which was not deemed possible. Please report
    /// this on [github](https://github.com/sbr075/envoke-rs/issues) to get it
    /// fixed!
    ///
    /// Thank you!
    #[error("fatal error occured")]
    Fatal,
}

#[derive(Debug, Error, strum::EnumIs)]
pub enum Error {
    #[error("retrieve error occured: {0}")]
    RetrieveError(#[from] RetrieveError),

    #[error("parse error occured: {0}")]
    ParseError(#[from] ParseError),

    #[error("failed to convert environment variable `{key}` to expected type")]
    ConvertError { key: String },
}
