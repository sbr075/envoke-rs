//! Run with
//!
//! ```not_rust
//! cargo run -p conditional_load
//! ```

#![allow(dead_code)]
use envoke::{Envoke, Fill};

#[derive(Debug, Fill, strum::EnumString)]
#[strum(serialize_all = "UPPERCASE")]
#[fill(rename_all = "SCREAMING_SNAKE_CASE")]
enum LogLevel {
    Debug,
    Info,
    #[fill(default)]
    Error,
}

#[derive(Debug, Fill)]
#[fill(rename_all = "SCREAMING_SNAKE_CASE")]
struct Production {
    #[fill(env, default = LogLevel::Error)]
    log_level: LogLevel,
}

#[derive(Debug, Fill)]
#[fill(rename_all = "SCREAMING_SNAKE_CASE")]
struct Staging {
    #[fill(env, default = LogLevel::Info)]
    log_level: LogLevel,
}

#[derive(Debug, Fill)]
#[fill(rename_all = "SCREAMING_SNAKE_CASE")]
struct Development {
    #[fill(env, default = LogLevel::Debug)]
    log_level: LogLevel,
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
    temp_env::with_vars([("ENVIRONMENT", Some("PRODUCTION"))], || {
        // or use `try_envoke()` for fail nice variant
        let mode = Mode::envoke();
        println!("{mode:?}");

        let log_level = LogLevel::envoke();
        println!("{log_level:?}");

        let env = Environment::envoke();
        println!("{env:#?}");
    })
}
