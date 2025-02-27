//! Run with
//!
//! ```not_rust
//! cargo run -p using_enums
//! ```

#![allow(dead_code)]
use envoke::{Envoke, Fill};

// Loading an enum will work as long as the enum impls the `FromStr` trait. In
// this example we are using [strum](https://crates.io/crates/strum) as its a
// convenient way to automatically implement these types of functions for us!
#[derive(Debug, strum::EnumString)]
#[strum(serialize_all = "lowercase")]
enum Mode {
    Development,
    Production,
}

#[derive(Debug, Fill)]
struct Environment {
    #[fill(env)]
    mode: Mode,
}

fn main() {
    temp_env::with_vars([("mode", Some("production"))], || {
        // or use `try_envoke()` for fail nice variant
        let env = Environment::envoke();
        println!("{env:#?}");
    })
}
