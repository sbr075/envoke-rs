//! Run with
//!
//! ```not_rust
//! cargo run -p loading_map_and_set_envs
//! ```

#![allow(dead_code)]
use std::collections::HashMap;

use envoke::{Envoke, Fill};

// Loading an enum will work as long as the enum impls the `FromStr` trait. In
// this example we are using [strum](https://crates.io/crates/strum) as its a
// convenient way to automatically implement these types of functions for us!
#[derive(Debug, strum::EnumString)]
#[strum(serialize_all = "snake_case")]
enum Status {
    Pending,
    InProgress,
    Completed,
    Failed,
    Cancelled,
}

#[derive(Debug, Fill)]
struct Environment {
    // The default delimiter is ","
    //
    // Note that `=` is reserved by the macro for differentiating key and values in a map
    #[fill(env)]
    map: HashMap<String, i32>,

    // but it can be overwritten by using the `delimiter` attribute
    #[fill(env, delimiter = ";")]
    set: Option<Vec<Status>>,
}

fn main() {
    temp_env::with_vars(
        [
            ("map", Some("key1=1,key2=2")),
            (
                "set",
                Some("pending;in_progress;completed;failed;cancelled"),
            ),
        ],
        || {
            // or use `try_envoke()` for fail nice variant
            let env = Environment::envoke();
            println!("{env:#?}");
        },
    )
}
