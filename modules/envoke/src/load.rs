use std::{
    collections::{BTreeMap, BTreeSet, HashMap, HashSet},
    env,
    fmt::Debug,
    hash::Hash,
    marker::PhantomData,
    str::FromStr,
};

use crate::errors::{ParseError, Result, RetrieveError};

fn load_once<T: FromStr, K: AsRef<str>>(keys: &[K]) -> Result<T> {
    for key in keys {
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
        keys: keys
            .iter()
            .map(|e| format!("`{}`", e.as_ref()))
            .collect::<Vec<String>>()
            .join(", "),
    })?
}

fn parse_map<K, V, M>(pairs: &str, delim: &str) -> std::result::Result<M, ParseError>
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

fn parse_set<T, S>(sequence: &str, delim: &str) -> std::result::Result<S, ParseError>
where
    T: FromStr,
    S: FromIterator<T>,
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

pub struct Envloader<T> {
    _marker: PhantomData<T>,
}

impl<K, V> Envloader<HashMap<K, V>>
where
    K: FromStr + Hash + Eq,
    V: FromStr,
{
    pub fn load_once<E>(keys: &[E], delim: &str) -> Result<HashMap<K, V>>
    where
        E: AsRef<str>,
    {
        let value: String = load_once(keys)?;
        parse_map(&value, delim).map_err(|e| e.into())
    }
}

impl<K, V> Envloader<BTreeMap<K, V>>
where
    K: FromStr + Ord,
    V: FromStr,
{
    pub fn load_once<E>(keys: &[E], delim: &str) -> Result<BTreeMap<K, V>>
    where
        E: AsRef<str>,
    {
        let value: String = load_once(keys)?;
        parse_map(&value, delim).map_err(|e| e.into())
    }
}

impl<T> Envloader<HashSet<T>>
where
    T: FromStr + Hash + Eq,
{
    pub fn load_once<E>(keys: &[E], delim: &str) -> Result<HashSet<T>>
    where
        E: AsRef<str>,
    {
        let value: String = load_once(keys)?;
        parse_set(&value, delim).map_err(|e| e.into())
    }
}

impl<T> Envloader<BTreeSet<T>>
where
    T: FromStr + Ord,
{
    pub fn load_once<E>(keys: &[E], delim: &str) -> Result<BTreeSet<T>>
    where
        E: AsRef<str>,
    {
        let value: String = load_once(keys)?;
        parse_set(&value, delim).map_err(|e| e.into())
    }
}

impl<T> Envloader<Vec<T>>
where
    T: FromStr,
{
    pub fn load_once<E>(keys: &[E], delim: &str) -> Result<Vec<T>>
    where
        E: AsRef<str>,
    {
        let value: String = load_once(keys)?;
        parse_set(&value, delim).map_err(|e| e.into())
    }
}

impl<T> Envloader<Option<T>>
where
    T: FromStr,
{
    /// Iterates through the list of environment variables and returns the first
    /// occurrence found
    pub fn load_once<K>(keys: &[K]) -> Result<Option<T>>
    where
        K: AsRef<str> + Debug,
    {
        load_once(keys).map(Some)
    }
}

pub trait Envload<T> {
    fn load_once<K>(keys: &[K]) -> Result<T>
    where
        K: AsRef<str> + Debug;
}

impl<T> Envload<T> for Envloader<T>
where
    T: FromStr,
{
    /// Iterates through the list of environment variables and returns the first
    /// occurrence found
    ///
    /// If no environment variables are found, the function panics
    fn load_once<K>(keys: &[K]) -> Result<T>
    where
        K: AsRef<str>,
    {
        load_once(keys)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_load_envs() {
        temp_env::with_vars(
            [
                ("KEY_1", Some("123")),
                ("KEY_2", Some("Hello, World!")),
                ("KEY_4", Some("Foo, bar!")),
                ("KEY_5", Some("hello, world,there!")),
            ],
            || {
                let key_1 = Envloader::<i32>::load_once(&["KEY_1"]);
                let key_2 = Envloader::<Option<String>>::load_once(&["KEY_2"]);
                let key_3 = Envloader::<Option<String>>::load_once(&["KEY_3"]);
                let key_4 = Envloader::<Option<String>>::load_once(&["KEY_3", "KEY_4"]);
                let key_5 = Envloader::<Vec<String>>::load_once(&["KEY_5"], ",");

                println!("{key_1:?}");
                println!("{key_2:?}");
                println!("{key_3:?}");
                println!("{key_4:?}");
                println!("{key_5:?}");

                assert!(1 == 2)
            },
        );
    }
}
