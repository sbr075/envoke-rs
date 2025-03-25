use std::{collections::HashMap, env, io::BufRead, str::FromStr};

use crate::errors::{ParseError, Result, RetrieveError};

pub fn load_dotenv(filepath: &str) -> Result<HashMap<String, String>> {
    let file = std::fs::File::open(filepath).unwrap();
    let reader = std::io::BufReader::new(file);

    let envs = reader
        .lines()
        .flat_map(|line| line.ok())
        .map(|line| line.trim().to_owned())
        .filter(|line| !line.is_empty() && !line.starts_with('#'))
        .filter_map(|line| {
            let (key, value) = line.split_once('=')?;
            let key = key.trim();
            let mut value = value.trim();

            // Remove optional surrounding quotes
            if let Some(stripped) = value.strip_prefix('"').and_then(|s| s.strip_suffix('"')) {
                value = stripped;
            }

            Some((key.to_string(), value.to_string()))
        })
        .collect();
    Ok(envs)
}

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

pub fn parse_str<V>(value: impl AsRef<str>) -> std::result::Result<V, ParseError>
where
    V: FromStr,
{
    let val = value.as_ref();
    val.parse().map_err(|_| ParseError::UnexpectedValueType {
        value: val.to_string(),
    })
}
