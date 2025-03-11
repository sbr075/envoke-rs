# envoke

Envoke is a simple and ergonomic way to load environment variables into struct fields with minimal setup. With support for multiple ways to define environment variables with additional fallbacks, default values, custom parsing, validation, and nested structs for a cleaner interface.

Designed to be flexible and reduce boilerplate, all to make it easier to manage environment variables in your application.

## Useful links
- [Documentation](https://docs.rs/envoke)
- [Github](https://github.com/sbr075/envoke-rs)
- [Examples](https://github.com/sbr075/envoke-rs/blob/main/examples/)
- [CHANGELOG](https://github.com/sbr075/envoke-rs/blob/main/CHANGELOG.md)

## Key features

- **Flexible Environment Variable Naming**  
  Customize naming conventions with support for multiple environments, renaming, prefixes, and suffixes.  

- **Versatile Default Values**  
  Define defaults through type-based values, direct assignments, or function return values.  

- **Pre-Assignment Parsing**  
  Convert values into complex types before field assignment for enhanced data handling.  

- **Comprehensive Validation**  
  Validate values before and/or after assignment to ensure data integrity.  

- **Structured Data Support**  
  Seamlessly work with nested structs, enums, or standalone enums.  

- **Built-in Sequence & Map Parsing**  
  Effortlessly parse sequence and map-formatted strings for structured data.

## Usage
Add to your `Cargo.toml`

```toml
[dependencies]
envoke = "0.2.2"
```

### Example

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

#### License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

</br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in this crate by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
</sub>