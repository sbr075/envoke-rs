# envoke-rs

A derive macro for loading environment variables into struct fields

---

[build-image]: https://github.com/sbr075/envoke-rs/actions/workflows/release.yml/badge.svg
[build]: https://github.com/sbr075/envoke-rs/actions/workflows/release.yml
[crates.io-image]: https://img.shields.io/badge/crates.io-envoke--rs-orange
[crates.io]: https://crates.io/crates/envoke-rs

</br>

# Useful links
- [Documentation](https://docs.rs/envoke)
- [Github](https://github.com/sbr075/envoke-rs)
- [Examples](https://github.com/sbr075/envoke-rs/blob/main/examples/)
- [CHANGELOG](https://github.com/sbr075/envoke-rs/blob/main/CHANGELOG.md)

</br>

# Features

- **Flexible Environment Variable Naming**  
  - Customize the case of environment variable names.  
  - Add global prefixes and suffixes (with per-field opt-out support).  

- **Multiple Ways to Define Environment Variables**  
  - Use the field name directly.  
  - Explicitly set the environment variable name to use.  
  - Define multiple environment variables as fallbacks. The order in which they are defined matters.  

- **Configurable Default Values**  
  - Support default values as field type defaults, static values, or function return values.  
  - Defaults can be used independently or as a fallback if no environment variable is found.  
  - Note that defaults are not run through validation or parsing functions.  

- **Custom Parsing Support**
  - Transform environment variable values before field assignment.  
    - Example: Convert a list of integers into a list of `Duration`.  

- **Value Validation**
  - Validate parsed values before assignment to ensure they meet expected constraints.  
    - Example: Ensure an integer is greater than `0`.  

- **Nested Struct Support**  
  - Populate multiple nested structs in a single call.  

- **Conditional Struct Loading**  
  - Dynamically load different structs based on an environment variable.  
  - Supports enums where each variant maps to a specific configuration struct.  
  - Enables flexible multi-environment configurations such as **Production**, **Staging**, and **Development**.  

- **Macro-Based Convenience**  
  - Works similarly to the `Default` trait—define only the fields you need.  
  - Unspecified fields are auto-filled, but even explicitly defined fields need to be able to be loaded correctly. 

</br>

# Usage
Add to your `Cargo.toml`

```toml
[dependencies]
envoke = "0.2.1"
```

## Example

```rust
use std::time::Duration;

use anyhow::Context;
use envoke::{Envoke, Error, Fill};

#[derive(Debug, Fill)]
#[fill(rename_all = "SCREAMING_SNAKE_CASE")]
struct Prod {
    #[fill(env)]
    api_port: u16,
}

#[derive(Debug, Fill)]
#[fill(rename_all = "SCREAMING_SNAKE_CASE")]
struct Development {
    #[fill(env)]
    api_port: u16,
}

#[derive(Debug, Fill)]
#[fill(rename_all = "UPPERCASE")]
enum Mode {
    Production(Production),
    Development(Development),
}

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
    #[fill(nested)]
    mode: Mode,

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

## Issues, new features, or contributions
If you discover any issues, find missing features that would make the crate better, or would like to contribute to the crate yourself go to the projects [GitHub](https://github.com/sbr075/envoke-rs) and open a new issue or pull request. In advance, thank you!

</br>

#### License

<sup>
Licensed under either of <a href="https://github.com/sbr075/envoke-rs/blob/main/LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="https://github.com/sbr075/envoke-rs/blob/main/LICENSE-MIT">MIT license</a> at your option.
</sup>

</br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in this crate by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
</sub>