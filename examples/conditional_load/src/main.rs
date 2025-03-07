//! Run with
//!
//! ```not_rust
//! cargo run -p conditional_load
//! ```

#![allow(dead_code)]
use envoke::{Envoke, Fill};

#[derive(Debug, Fill)]
#[fill(
    rename_all = "SCREAMING_SNAKE_CASE",
    prefix = "PRODUCTION",
    delimiter = "_"
)]
struct Production {
    #[fill(env)]
    api_port: u16,
}

#[derive(Debug, Fill)]
#[fill(
    rename_all = "SCREAMING_SNAKE_CASE",
    prefix = "STAGING",
    delimiter = "_"
)]
struct Staging {
    #[fill(env)]
    api_port: u16,
}

#[derive(Debug, Fill)]
#[fill(
    rename_all = "SCREAMING_SNAKE_CASE",
    prefix = "DEVELOPMENT",
    delimiter = "_"
)]
struct Development {
    #[fill(env)]
    api_port: u16,
}

#[derive(Debug, Fill)]
#[fill(rename_all = "UPPERCASE", env = "ENVIRONMENT")]
enum Mode {
    Production(Production),
    Staging(Staging),
    Development(Development),
}

#[derive(Debug, Fill)]
pub struct Environment {
    #[fill(nested)]
    mode: Mode,
}

fn main() {
    temp_env::with_vars(
        [
            ("ENVIRONMENT", Some("PRODUCTION")),
            ("PRODUCTION_API_PORT", Some("8000")),
            ("STAGING_API_PORT", Some("8001")),
            ("DEVELOPMENT_API_PORT", Some("8002")),
        ],
        || {
            // or use `try_envoke()` for fail nice variant
            let mode = Mode::envoke();
            println!("{mode:#?}");

            let env = Environment::envoke();
            println!("{env:#?}");
        },
    )
}
