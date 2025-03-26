//! Run with
//!
//! ```not_rust
//! cargo run -p parsing_envs
//! ```

#![allow(dead_code)]
use std::time::Duration;

use envoke::{Envoke, Fill};

#[derive(Debug, Fill)]
struct Environment {
    // Note that default values are not parsed as they are expected to already be the correct type
    #[fill(env, default = "http://localhost:8080")]
    url: String,

    #[fill(env, parse_fn = Duration::from_secs, arg_type = u64)]
    connect_timeout: Duration,
}

fn main() {
    temp_env::with_vars([("connect_timeout", Some("30"))], || {
        // or use `try_envoke()` for fail nice variant
        let env = Environment::envoke();
        println!("{env:#?}");
    })
}
