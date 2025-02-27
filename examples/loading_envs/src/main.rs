//! Run with
//!
//! ```not_rust
//! cargo run -p loading_envs
//! ```

#![allow(dead_code)]
use envoke::{Envoke, Fill};

#[derive(Debug, Fill)]
struct Environment {
    // Will look for an environment variable with the name: `name`
    #[fill(env)]
    name: String,

    // Will look for an environment variable with the name: `age`
    #[fill(env = "age")]
    age: u16,

    // Will look for an environment variables with either the name: `location` or `position`
    // Note that the order matters so `location` will be prioritized over `position`
    #[fill(env, env = "position")]
    location: String,
}

fn main() {
    temp_env::with_vars(
        [
            ("name", Some("John")),
            ("age", Some("50")),
            ("position", Some("The world")),
        ],
        || {
            // or use `try_envoke()` for fail nice variant
            let env = Environment::envoke();
            println!("{env:#?}");
        },
    )
}
