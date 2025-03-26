//! Run with
//!
//! ```not_rust
//! cargo run -p from_dotenv
//! ```

#![allow(dead_code)]

use envoke::{Envoke, Fill};

#[derive(Debug, Fill)]
#[fill(dotenv = ".env", rename_all = "UPPERCASE")]
enum Status {
    Employed,
    Unemployed,
}

#[derive(Debug, Fill)]
#[fill(dotenv = ".env", rename_all = "UPPERCASE")]
struct Environment {
    #[fill(env)]
    name: String,

    age: u16,

    // This wont look for an environment variable in the dotenv as its specified
    // only a default is used for this field.
    #[fill(default = "The world")]
    location: String,

    // Uses the envvar found in the processes environment as it has priority
    // over what is found in the dotenv file.
    profession: String,

    #[fill(nested)]
    status: Status,
}

fn main() {
    temp_env::with_var("PROFESSION", Some("DEV"), || {
        let env = Environment::envoke();
        println!("{env:#?}");

        let status = Status::envoke();
        println!("{status:#?}");
    });
}
