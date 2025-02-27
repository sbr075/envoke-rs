use std::{marker::PhantomData, str::FromStr};

use crate::{
    errors::Result,
    utils::{load_once, parse_map, parse_set},
};

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

impl<V> Envloader<V>
where
    V: FromStr,
{
    pub fn load_once(envs: &[impl AsRef<str>], _delim: &str) -> Result<V> {
        load_once(envs)
    }
}
