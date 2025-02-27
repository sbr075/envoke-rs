use std::{marker::PhantomData, str::FromStr};

use crate::{
    errors::Result,
    utils::{load_once, parse_map, parse_set},
};

pub struct OptEnvloader<T> {
    _marker: PhantomData<T>,
}

pub trait FromMapOpt<M, K, V> {
    fn load_once(envs: &[impl AsRef<str>], _delim: &str) -> Result<Option<M>>;
}

impl<M, K, V> FromMapOpt<M, K, V> for OptEnvloader<Option<M>>
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

pub trait FromSetOpt<S, V> {
    fn load_once(envs: &[impl AsRef<str>], _delim: &str) -> Result<Option<S>>;
}

impl<S, V> FromSetOpt<S, V> for OptEnvloader<Option<S>>
where
    V: FromStr,
    S: FromIterator<V>,
{
    fn load_once(envs: &[impl AsRef<str>], delim: &str) -> Result<Option<S>> {
        let value: String = load_once(envs)?;
        parse_set(&value, delim).map(Some).map_err(|e| e.into())
    }
}

impl<V> OptEnvloader<Option<V>>
where
    V: FromStr,
{
    pub fn load_once(envs: &[impl AsRef<str>], _delim: &str) -> Result<Option<V>> {
        Ok(load_once(envs).ok())
    }
}
