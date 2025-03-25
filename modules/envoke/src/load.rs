use std::{collections::HashMap, marker::PhantomData, str::FromStr};

use crate::{
    errors::Result,
    utils::{load_once, parse_map, parse_set, parse_str},
};

pub struct Envloader<T> {
    _marker: PhantomData<T>,
}

pub trait FromMap<M, K, V> {
    fn load_once(
        envs: &[impl AsRef<str>],
        delim: &str,
        fallback: Option<&HashMap<String, String>>,
    ) -> Result<M>;
}

impl<M, K, V> FromMap<M, K, V> for Envloader<M>
where
    K: FromStr,
    V: FromStr,
    M: FromIterator<(K, V)>,
{
    fn load_once(
        envs: &[impl AsRef<str>],
        delim: &str,
        fallback: Option<&HashMap<String, String>>,
    ) -> Result<M> {
        let value: String = match load_once(envs) {
            Ok(value) => value,
            Err(e) => match fallback.and_then(|f| envs.iter().find_map(|e| f.get(e.as_ref()))) {
                Some(value) => value.to_owned(),
                None => return Err(e),
            },
        };

        parse_map(&value, delim).map_err(|e| e.into())
    }
}

pub trait FromSet<S, V> {
    fn load_once(
        envs: &[impl AsRef<str>],
        delim: &str,
        fallback: Option<&HashMap<String, String>>,
    ) -> Result<S>;
}

impl<S, V> FromSet<S, V> for Envloader<S>
where
    V: FromStr,
    S: FromIterator<V>,
{
    fn load_once(
        envs: &[impl AsRef<str>],
        delim: &str,
        fallback: Option<&HashMap<String, String>>,
    ) -> Result<S> {
        let value: String = match load_once(envs) {
            Ok(value) => value,
            Err(e) => match fallback.and_then(|f| envs.iter().find_map(|e| f.get(e.as_ref()))) {
                Some(value) => value.to_owned(),
                None => return Err(e),
            },
        };

        parse_set(&value, delim).map_err(Into::into)
    }
}

impl<V> Envloader<V>
where
    V: FromStr,
{
    pub fn load_once(
        envs: &[impl AsRef<str>],
        _delim: &str,
        fallback: Option<&HashMap<String, String>>,
    ) -> Result<V> {
        load_once(envs).or_else(|e| {
            fallback
                .and_then(|f| envs.iter().find_map(|e| f.get(e.as_ref())))
                .map_or(Err(e), |val| parse_str(val).map_err(Into::into))
        })
    }
}
