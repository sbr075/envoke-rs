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

# Key features

- **Flexible Environment Variable Naming**  
  Customize naming conventions with support for multiple environments, renaming, prefixes, suffixes, etc.  

- **Versatile Default Values**  
  Define defaults through type's default, direct assignments, or function return values.  

- **Pre-Assignment Parsing**  
  Convert values into complex types before field assignment for enhanced data handling.  

- **Comprehensive Validation**  
  Validate values before and/or after assignment to ensure data integrity.  

- **Structured Data Support**  
  Seamlessly work with nested structs, enums, or standalone enums.  

- **Built-in Sequence & Map Parsing**  
  Effortlessly parse sequence and map-formatted strings for structured data.

</br>

# Usage
Add to your `Cargo.toml`

```toml
[dependencies]
envoke = "0.3.0"
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