# envoke

Envoke is a simple and ergonomic way to load environment variables into struct fields with minimal setup. With support for multiple ways to define environment variables with additional fallbacks, default values, custom parsing, validation, and nested structs for a cleaner interface.

Designed to be flexible and reduce boilerplate, all to make it easier to manage environment variables in your application.

## Useful links
- [Documentation](https://docs.rs/envoke)
- [Github](https://github.com/sbr075/envoke-rs)
- [Examples](https://github.com/sbr075/envoke-rs/blob/main/examples/)
- [CHANGELOG](https://github.com/sbr075/envoke-rs/blob/main/CHANGELOG.md)

## Features

Examples of these features can be found in the [docs](https://docs.rs/envoke)

- **Flexible Environment Variable Naming:**  
  - Customize the case of environment variable names.  
  - Add global prefixes and suffixes (with per-field opt-out support).  

- **Multiple Ways to Define Environment Variables:**  
  - Use the field name directly.  
  - Explicitly set the environment variable name to use.  
  - Define multiple environment variables as fallbacks. The order in which they are defined matters.  

- **Configurable Default Values:**  
  - Support default values as field type defaults, static values, or function return values.  
  - Defaults can be used independently or as a fallback if no environment variable is found.  
  - Note that defaults are not ran through validation or parsing functions.

- **Custom Parsing Support:**  
  - Transform environment variable values before field assignment.  
    - Example: Convert a list of integers into a list of `Duration`.  

- **Value Validation:**  
  - Validate parsed values before assignment to ensure they meet expected constraints.  
    - Example: Ensure an integer is greater than 0.  

- **Nested Struct Support:**  
  - Populate multiple nested structs in a single call.  

- **Macro-Based Convenience:**  
  - Works similarly to the `Default` trait—define only the fields you need.  
    - Unspecified fields are auto-filled, but explicitly defined fields must be correctly populated.  


## Usage
Add to your `Cargo.toml`
```toml
[dependencies]
envoke = "0.1.4"
```

### Example

```rust
use std::time::Duration;

use anyhow::Context;
use envoke::{Envoke, Error, Fill};

fn above_thirty(secs: &u64) -> anyhow::Result<()> {
    if *secs < 30 {
        anyhow::bail!("connect timeout cannot be less than 30 seconds, found {secs} second(s)")
    }

    Ok(())
}

fn to_duration(secs: u64) -> Duration {
    Duration::from_secs(secs)
}

#[derive(Debug, Fill)]
#[fill(rename_all = "SCREAMING_SNAKE_CASE")]
struct Environment {
    #[fill(env, default = Duration::from_secs(30))]
    #[fill(parse_fn = to_duration, arg_type = u64, validate_fn(before = above_thirty))]
    connect_timeout: Duration,
}

fn main() -> anyhow::Result<()> {
    let env =
        Environment::try_envoke().with_context(|| "An error occurred while loading environment");

    ...
}
```

</br>

#### License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

</br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in this crate by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
</sub>