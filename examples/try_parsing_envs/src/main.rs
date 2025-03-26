//! Run with
//!
//! ```not_rust
//! cargo run -p try_parsing_envs
//! ```

#![allow(dead_code)]

use envoke::{Envoke, Fill};
use uuid::Uuid;

fn to_uuid(s: String) -> anyhow::Result<Uuid> {
    let uuid = Uuid::parse_str(&s)?;
    Ok(uuid)
}

#[derive(Debug, Fill)]
struct Environment {
    #[fill(env, try_parse_fn = to_uuid, arg_type = String)]
    uuid: Uuid,
}

fn main() {
    temp_env::with_vars(
        [("uuid", Some("afb922e7-e8b0-40d3-b6fe-9016fa244f64"))],
        || {
            // or use `try_envoke()` for fail nice variant
            let env = Environment::envoke();
            println!("{env:#?}");
        },
    )
}
