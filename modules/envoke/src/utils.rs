use std::{env, str::FromStr};

use crate::errors::{ParseError, Result, RetrieveError};

pub fn load_once<T: FromStr>(envs: &[impl AsRef<str>]) -> Result<T> {
    for key in envs {
        let key = key.as_ref().trim();

        let value = match env::var(key) {
            Ok(value) => value,
            Err(e) => match e {
                env::VarError::NotPresent => continue,
                env::VarError::NotUnicode(_) => {
                    return Err(RetrieveError::InvalidUnicode {
                        key: key.to_string(),
                    })?
                }
            },
        };

        return match value.trim().parse() {
            Ok(value) => Ok(value),
            Err(_) => Err(ParseError::UnexpectedValueType { value })?,
        };
    }

    Err(RetrieveError::NotFound {
        keys: envs
            .iter()
            .map(|e| format!("`{}`", e.as_ref()))
            .collect::<Vec<String>>()
            .join(", "),
    })?
}

pub fn parse_map<K, V, M>(pairs: &str, delim: &str) -> std::result::Result<M, ParseError>
where
    K: FromStr,
    V: FromStr,
    M: FromIterator<(K, V)>,
{
    pairs
        .trim()
        .split(delim)
        .map(|part| {
            let mut parts = part.splitn(2, "=");
            let key = parts.next().ok_or(ParseError::MissingKey)?.trim();
            let val = parts.next().ok_or(ParseError::MissingValue)?.trim();

            if key.is_empty() {
                return Err(ParseError::MissingKey);
            }

            if val.is_empty() {
                return Err(ParseError::MissingValue);
            }

            let parsed_key: K = key.parse().map_err(|_| ParseError::UnexpectedKeyType {
                key: key.to_string(),
            })?;
            let parsed_val = val.parse().map_err(|_| ParseError::UnexpectedValueType {
                value: val.to_string(),
            })?;

            Ok((parsed_key, parsed_val))
        })
        .collect()
}

pub fn parse_set<S, V>(sequence: &str, delim: &str) -> std::result::Result<S, ParseError>
where
    V: FromStr,
    S: FromIterator<V>,
{
    sequence
        .trim()
        .split(delim)
        .map(|part| {
            let val = part.trim();
            if val.is_empty() {
                return Err(ParseError::MissingValue);
            }

            val.parse().map_err(|_| ParseError::UnexpectedValueType {
                value: val.to_string(),
            })
        })
        .collect()
}
