use std::{env, marker::PhantomData, str::FromStr};

use crate::errors::{ParseError, Result, RetrieveError};

fn load_once<T: FromStr>(envs: &[impl AsRef<str>]) -> Result<T> {
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

fn parse_set<S, V>(sequence: &str, delim: &str) -> std::result::Result<S, ParseError>
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

pub struct Envloader<T> {
    _marker: PhantomData<T>,
}

pub trait FromMap<M, K, V> {
    fn load_once(envs: &[impl AsRef<str>], _delim: &str) -> Result<M>;
}

impl<M, K, V> FromMap<M, K, V> for Envloader<M>
where
    K: FromStr,
    V: FromStr,
    M: FromIterator<(K, V)>,
{
    fn load_once(envs: &[impl AsRef<str>], delim: &str) -> Result<M> {
        let value: String = load_once(envs)?;
        parse_map(&value, delim).map_err(|e| e.into())
    }
}

pub trait FromMapOpt<M, K, V> {
    fn load_once(envs: &[impl AsRef<str>], _delim: &str) -> Result<Option<M>>;
}

impl<M, K, V> FromMapOpt<M, K, V> for Envloader<Option<M>>
where
    K: FromStr,
    V: FromStr,
    M: FromIterator<(K, V)>,
{
    fn load_once(envs: &[impl AsRef<str>], delim: &str) -> Result<Option<M>> {
        let value: String = load_once(envs)?;
        parse_map(&value, delim).map(Some).map_err(|e| e.into())
    }
}

pub trait FromSet<S, V> {
    fn load_once(envs: &[impl AsRef<str>], _delim: &str) -> Result<S>;
}

impl<S, V> FromSet<S, V> for Envloader<S>
where
    V: FromStr,
    S: FromIterator<V>,
{
    fn load_once(envs: &[impl AsRef<str>], delim: &str) -> Result<S> {
        let value: String = load_once(envs)?;
        parse_set(&value, delim).map_err(|e| e.into())
    }
}

pub trait FromSetOpt<S, V> {
    fn load_once(envs: &[impl AsRef<str>], _delim: &str) -> Result<Option<S>>;
}

impl<S, V> FromSetOpt<S, V> for Envloader<Option<S>>
where
    V: FromStr,
    S: FromIterator<V>,
{
    fn load_once(envs: &[impl AsRef<str>], delim: &str) -> Result<Option<S>> {
        let value: String = load_once(envs)?;
        parse_set(&value, delim).map(Some).map_err(|e| e.into())
    }
}

pub trait FromSingleOpt<V> {
    fn load_once(envs: &[impl AsRef<str>], _delim: &str) -> Result<Option<V>>;
}

impl<V> FromSingleOpt<V> for Envloader<Option<V>>
where
    V: FromStr,
{
    fn load_once(envs: &[impl AsRef<str>], _delim: &str) -> Result<Option<V>> {
        load_once(envs).map(Some)
    }
}

impl<V> Envloader<V>
where
    V: FromStr,
{
    pub fn load_once(envs: &[impl AsRef<str>], _delim: &str) -> Result<V> {
        load_once(envs)
    }
}

#[cfg(test)]
mod test {
    use std::collections::{HashMap, HashSet};

    use super::{Envloader, FromMap, FromMapOpt, FromSet, FromSetOpt, FromSingleOpt};

    #[test]
    fn test_load_envs() {
        temp_env::with_vars(
            [
                ("KEY_0", Some("Hello")),
                ("KEY_1", Some("123")),
                ("KEY_2", Some("Hello, World!")),
                ("KEY_4", Some("Foo, bar!")),
                ("KEY_5", Some("hello, world,there!")),
                ("KEY_6", Some("key1=value1,key2=value2")),
            ],
            || {
                let key_0 = Envloader::<String>::load_once(&["KEY_0"], ",");
                let key_1 = Envloader::<i32>::load_once(&["KEY_1"], ",");
                let key_2 = <Envloader<Option<String>> as FromSingleOpt<String>>::load_once(
                    &["KEY_2"],
                    ",",
                );
                let key_3 = <Envloader<Option<String>> as FromSingleOpt<String>>::load_once(
                    &["KEY_3"],
                    ",",
                );
                let key_4 = <Envloader<Option<String>> as FromSingleOpt<String>>::load_once(
                    &["KEY_3", "KEY_4"],
                    ",",
                );
                let key_5 = Envloader::<Vec<String>>::load_once(&["KEY_5"], ",");
                let key_6 = Envloader::<HashMap<String, String>>::load_once(&["KEY_6"], ",");
                let key_7 =
                    Envloader::<Option<HashMap<String, String>>>::load_once(&["KEY_7"], ",");
                let key_8 = Envloader::<Option<HashSet<String>>>::load_once(&["KEY_8"], ",");

                println!("{key_0:?}");
                println!("{key_1:?}");
                println!("{key_2:?}");
                println!("{key_3:?}");
                println!("{key_4:?}");
                println!("{key_5:?}");
                println!("{key_6:?}");
                println!("{key_7:?}");
                println!("{key_8:?}");

                assert!(1 == 2)
            },
        );
    }
}
