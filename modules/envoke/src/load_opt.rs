use std::{collections::HashMap, marker::PhantomData, str::FromStr};

use crate::{
    errors::Result,
    utils::{load_once, parse_map, parse_set, parse_str},
};

pub struct OptEnvloader<T> {
    _marker: PhantomData<T>,
}

pub trait FromMapOpt<M, K, V> {
    fn load_once(
        envs: &[impl AsRef<str>],
        delim: &str,
        fallback: Option<&HashMap<String, String>>,
    ) -> Result<Option<M>>;
}

impl<M, K, V> FromMapOpt<M, K, V> for OptEnvloader<Option<M>>
where
    K: FromStr,
    V: FromStr,
    M: FromIterator<(K, V)>,
{
    fn load_once(
        envs: &[impl AsRef<str>],
        delim: &str,
        fallback: Option<&HashMap<String, String>>,
    ) -> Result<Option<M>> {
        let value: String = match load_once(envs) {
            Ok(value) => value,
            Err(_) => match fallback.and_then(|f| envs.iter().find_map(|e| f.get(e.as_ref()))) {
                Some(value) => value.to_owned(),
                None => return Ok(None),
            },
        };

        parse_map(&value, delim).map(Some).map_err(|e| e.into())
    }
}

pub trait FromSetOpt<S, V> {
    fn load_once(
        envs: &[impl AsRef<str>],
        delim: &str,
        fallback: Option<&HashMap<String, String>>,
    ) -> Result<Option<S>>;
}

impl<S, V> FromSetOpt<S, V> for OptEnvloader<Option<S>>
where
    V: FromStr,
    S: FromIterator<V>,
{
    fn load_once(
        envs: &[impl AsRef<str>],
        delim: &str,
        fallback: Option<&HashMap<String, String>>,
    ) -> Result<Option<S>> {
        let value: String = match load_once(envs) {
            Ok(value) => value,
            Err(_) => match fallback.and_then(|f| envs.iter().find_map(|e| f.get(e.as_ref()))) {
                Some(value) => value.to_owned(),
                None => return Ok(None),
            },
        };

        parse_set(&value, delim).map(Some).map_err(|e| e.into())
    }
}

impl<V> OptEnvloader<Option<V>>
where
    V: FromStr,
{
    pub fn load_once(
        envs: &[impl AsRef<str>],
        _delim: &str,
        fallback: Option<&HashMap<String, String>>,
    ) -> Result<Option<V>> {
        load_once(envs).map(Some).or_else(|e| {
            fallback
                .and_then(|f| envs.iter().find_map(|e| f.get(e.as_ref())))
                .map(parse_str)
                .transpose()
                .or(Err(e))
        })
    }
}
