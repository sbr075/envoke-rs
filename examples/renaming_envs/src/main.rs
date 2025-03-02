//! Run with
//!
//! ```not_rust
//! cargo run -p renaming_envs
//! ```

#![allow(dead_code)]
use envoke::{Envoke, Fill};

// A delimiter isn't strictly needed but it will ensure the prefix and suffix is
// separated from the original name
#[derive(Debug, Fill)]
#[fill(
    rename_all = "SCREAMING_SNAKE_CASE",
    delimiter = "_",
    prefix = "prefix",
    suffix = "suffix"
)]
struct Environment {
    #[fill(env = "test")]
    full: String,

    // You can opt out of using the global prefix by assigning the field the `no_prefix` attribute
    #[fill(env = "test", no_prefix)]
    no_prefix: String,

    // You can opt out of using the global suffix by assigning the field the `no_suffix` attribute
    #[fill(env = "test", no_suffix)]
    no_suffix: String,

    // You can opt out of using both the prefix and suffix by assigning the field both attributes
    #[fill(env = "test", no_prefix, no_suffix)]
    nothing: String,
}

fn main() {
    temp_env::with_vars(
        [
            (
                "PREFIX_TEST_SUFFIX",
                Some("I have a both a prefix and suffix"),
            ),
            ("TEST_SUFFIX", Some("I don't have a prefix")),
            ("PREFIX_TEST", Some("I don't have a suffix")),
            ("TEST", Some("I don't have a prefix or suffix")),
        ],
        || {
            // or use `try_envoke()` for fail nice variant
            let env = Environment::envoke();
            println!("{env:#?}");
        },
    )
}
