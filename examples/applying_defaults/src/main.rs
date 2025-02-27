//! Run with
//!
//! ```not_rust
//! cargo run -p applying_defaults
//! ```

#![allow(dead_code)]
use envoke::{Envoke, Fill};

#[derive(Debug, Default)]
enum Mode {
    Development,
    #[default]
    Production,
}

fn max_connections() -> u16 {
    // Maybe some calculations of how many could be allowed to connect at once
    // depending on the network bandwidth
    10
}

#[derive(Debug, Fill)]
struct Environment {
    // Will look for an environment variable with the name: `name`
    #[fill(env, default = "webserver")]
    service: String,

    // Will look for an environment variable with the name: `age`
    #[fill(default)]
    mode: Mode,

    #[fill(env, default = max_connections())]
    max_connections: u16,
}

fn main() {
    // or use `try_envoke()` for fail nice variant
    let env = Environment::envoke();
    println!("{env:#?}");
}
