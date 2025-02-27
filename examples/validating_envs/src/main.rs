//! Run with
//!
//! ```not_rust
//! cargo run -p validating_envs
//! ```

#![allow(dead_code)]
use std::time::Duration;

use envoke::{Envoke, Fill};

fn equal_or_greater_than_thirty(secs: &u64) -> anyhow::Result<()> {
    if *secs < 30 {
        anyhow::bail!("connect timeout cannot be less than 30 seconds")
    }

    Ok(())
}

fn to_duration(secs: u64) -> Duration {
    Duration::from_secs(secs)
}

fn is_http(url: &str) -> anyhow::Result<()> {
    if !url.starts_with("http://") {
        anyhow::bail!("invalid URL: expected 'http://' schema")
    }

    Ok(())
}

#[derive(Debug, Fill)]
struct Environment {
    // Directly assigning `is_http` to validate fn is the same as doing validate_fn(after =
    // equal_or_greater_than_thirty)
    //
    // Note that default values are not validated as they are expected to be correct
    #[fill(env, validate_fn = is_http)]
    url: String,

    // Note that you can use both after and before like validate_fn(before = ..., after = ...)
    #[fill(env, parse_fn = to_duration, arg_type = u64, validate_fn(before = equal_or_greater_than_thirty))]
    connect_timeout: Duration,
}

fn main() {
    // Try changing these values to see the function panic!
    temp_env::with_vars(
        [
            ("url", Some("http://localhost:8080")),
            ("connect_timeout", Some("30")),
        ],
        || {
            // or use `try_envoke()` for fail nice variant
            let env = Environment::envoke();
            println!("{env:#?}");
        },
    )
}
