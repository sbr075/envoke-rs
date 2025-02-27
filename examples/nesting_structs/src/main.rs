//! Run with
//!
//! ```not_rust
//! cargo run -p nesting_structs
//! ```

#![allow(dead_code)]
use envoke::{Envoke, Fill};

#[derive(Debug, Default)]
enum Mode {
    Development,
    #[default]
    Production,
}

#[derive(Debug, Fill)]
struct ServerSettings {
    #[fill(env, default = "http://localhost:8080")]
    url: String,

    #[fill(env, default = 10)]
    max_connections: u64,
}

#[derive(Debug, Fill)]
struct Environment {
    #[fill(default)]
    mode: Mode,

    #[fill(nested)]
    server_settings: ServerSettings,
}

fn main() {
    // or use `try_envoke()` for fail nice variant
    let env = Environment::envoke();
    println!("{env:#?}");
}
