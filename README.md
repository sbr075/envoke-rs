# envoke-rs

A derive macro for loading environment variables into struct fields

---

[build-image]: https://github.com/sbr075/envoke-rs/actions/workflows/release.yml/badge.svg
[build]: https://github.com/sbr075/envoke-rs/actions/workflows/release.yml
[crates.io-image]: https://img.shields.io/badge/crates.io-envoke--rs-orange
[crates.io]: https://crates.io/crates/envoke-rs

</br>

# Usage
Add to your `Cargo.toml`
```toml
[dependencies]
envoke = "0.1.7"
```

or run `cargo add envoke`

## Useful links
- [Documentation](https://docs.rs/envoke)
- [Github](https://github.com/sbr075/envoke-rs)
- [Examples](https://github.com/sbr075/envoke-rs/blob/main/examples/)
- [CHANGELOG](https://github.com/sbr075/envoke-rs/blob/main/CHANGELOG.md)

</br>

# Attributes
## Container

Container attributes allows you to globally configure how environment variable name are transformed before retrieving from the processe's environment.

### Options
| Attribute    | Default | Description                                                                                                                                                                                                                                                                                                                                                                                  |
| ------------ | ------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `prefix`     | None    | Set a custom prefix which will be prepended infront of environment variables before fetching                                                                                                                                                                                                                                                                                                 |
| `suffix`     | None    | Set a custom prefix which will be appended infront of environment variables before fetching                                                                                                                                                                                                                                                                                                  |
| `delimiter`  | None    | Set a customer delimiter used for separated prefix, environment variable, and suffix. **NB!** If you are using the `rename_all` attribute as well it will take priority over the delimiter. It can still be useful to include the delimiter to ensure the prefix, environment variable, and suffix are separated before renaming occurs otherwise they will be interpreted as a single word! |
| `rename_all` | None    | Rename all environment variables to a different naming case. See [name cases](#name-cases) for a full list and description of the different options.                                                                                                                                                                                                                                         |

If there are any more attributes you think would be useful open an issue and I will look at it when I have time! - Thanks

</br>

### Name cases
| Case                 | Value                  | Description                                                                                                        |
| -------------------- | ---------------------- | ------------------------------------------------------------------------------------------------------------------ |
| Lower case           | `lowercase` or `lower` | Converts all characters to lowercase and removes binding characters                                                |
| Upper case           | `UPPERCASE` or `UPPER` | Converts all characters to uppercase and removes binding characters                                                |
| Pascal case          | `PascalCase`           | Capitalizes the first letter of each word and removes binding                                                      |
| Camel case           | `camelCase`            | Lowercases the first letter but capitalizes the first letter of subsequent words while removing binding characters |
| Snake case           | `snake_case`           | Converts names to lowercase and uses underscores `_` to separate words                                             |
| Screaming snake case | `SCREAMING_SNAKE_CASE` | Converts names to uppercase and uses underscores `_` to separate words                                             |
| Kebab case           | `kebab-case`           | Converts names to lowercase and uses hyphens `-` to separate words                                                 |
| Screaming kebab case | `SCREAMING-KEBAB-CASE` | Converts names to uppercase and uses hyphens `-` to separate words                                                 |

</br>

### Examples

```rust
use envoke::Fill;

#[derive(Fill)]
#[fill(prefix = "prefix", delimiter = "_", suffix = "suffix", rename_all = "camelCase")]
struct Example {
    #[fill(env = "EXAMPLE_ENV")]
    field: String,
}

fn main() {
    let _ = Example::envoke();
}
```

This will cause the macro to attempt to fill the field `field` with the value from the environment variable `prefixExampleEnvSuffix`.

</br>

## Field

Field attributes allows you to configure how each field is individually handled when it comes to setting the fields value.

### Options
| Attribute     | Default   | Description                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| ------------- | --------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `env`         | None      | Environment variable name to load the field value from. Can be chained multiple times to allow for fallbacks. The macro follows a first come, first serve basis meaning it attempts to load the variables in the order they are listed. Once an value is found it will try to parse it into the specified type. If it fails it will return an error and wont try the remaining ones in the list. This behavior might change in the future. Optionally, you can supply your own parsing function. See `parse_fn` for more information! |
| `default`     | None      | Use the default value if the environment variable is not found. Optionally to statically assign a value to the field `env` can be omitted.                                                                                                                                                                                                                                                                                                                                                                                            |
| `parse_fn`    | None      | Set a custom parsing function for parsing the retrieved value before assigning it to the field. This can be useful when the fields type does not implement the `FromStr` trait. Requires `arg_type` to be set                                                                                                                                                                                                                                                                                                                         |
| `arg_type`    | None      | Specify the argument type which the `parse_fn` function requires. As I don't know if it is possible to find the type automatically this argument is required such that the environment variable value can be parsed into the expected type first before being set as the argument in the function call.                                                                                                                                                                                                                               |
| `validate_fn` | None      | Set a custom validation function for ensuring the loaded value meets expectations. Note `validate_fn` supports both direct assignment and parentheses assignments. See [example](#validating-a-loaded-value)                                                                                                                                                                                                                                                                                                                          |
| `delimiter`   | Comma (,) | Used when parsing environment variable which is a stringified map or set. The delimiter specifies the boundary between values.                                                                                                                                                                                                                                                                                                                                                                                                        |
| `no_prefix`   | False     | Disable adding the global prefix to this environment variable. This will also remove the delimiter that wouldn't normally be between the environment variable and prefix                                                                                                                                                                                                                                                                                                                                                              |
| `no_suffix`   | False     | Disable adding the global suffix to this environment variable. This will also remove the delimiter that wouldn't normally be between the environment variable and suffix                                                                                                                                                                                                                                                                                                                                                              |
| `nested`      | False     | Indicate that the field is a struct. Required when the field type is another struct                                                                                                                                                                                                                                                                                                                                                                                                                                                   |

</br>

### Examples

#### Loading an environment variable

Below is an example of fully and partially setting a structs fields. 

```rust
use envoke::{Envoke, Fill};

#[derive(Fill)]
struct Example {
    #[fill(env, env = "ENV_STRING", default)]
    field1: String,

    #[fill(env = "ENV_INT")]
    field2: i32,
}

fn main() {
    temp_env::with_vars(
        [
            ("field1", Some("Hello, Reader!")),
            ("ENV_INT", Some("123")),
        ],
        || {
            // Note that due to limitations of the macro it cannot recognize which
            // fields are already set, and as such will try to fetch `field1`. Use the
            // `default` attribute to prevent any errors.
            let _ = Example {
                field1: "Hello, Reader!".to_string(),
                ..Envoke::envoke()
            };

            // Fill struct fields from the environment variables
            let _ = Example::envoke();
        },
    );
}
```

</br>

#### Loading an environment variable with fallbacks

Below is an example of setting a field name with multiple fallbacks incase the first environment variable does not exist.

```rust
use envoke::{Envoke, Fill};

#[derive(Fill)]
struct Example {
    #[fill(env = "ENV_STRING1", env = "ENV_STRING2")]
    field1: String,
}

fn main() {
    temp_env::with_vars(
        [
            ("ANOTHER_ENV"), Some("Not what we are looking for"),
            ("ENV_STRING2"), Some("Actually exists"),
        ],
        || {
            // Fills struct field with `Actually exists`
            let _ = Example::envoke();
        },
    );
}
```

</br>

#### Loading an environment variable with default fallback

Using the field type's default value

```rust
use envoke::{Envoke, Fill};

#[derive(Fill)]
struct Example {
    #[fill(default)]
    field1: i32,
}

fn main() {
    // Fills the struct field with i32::default() (0)
    let _ = Example::envoke();
}
```

</br>

Directly assigning a value

```rust
use envoke::{Envoke, Fill};

#[derive(Fill)]
struct Example {
    #[fill(default = 10)]
    field1: i32,
}

fn main() {
    // Fills the struct field with 10
    let _ = Example::envoke();
}
```

</br>

Assigning a value from the return value of a function

```rust
use std::time::Duration;

use envoke::{Envoke, Fill};

#[derive(Fill)]
struct Example {
    #[fill(default = default_duration())]
    field1: Duration,

    #[fill(default = default_duration_with_arg(5))]
    field2: DateTime<Utc>,
}

fn default_duration() -> Duration {
    Duration::from_secs(10)
}

fn default_duration_with_arg(add: u64) -> Duration {
    Duration::from_secs(10 + add)
}

fn main() {
    // Fills the struct field with Duration 10s and Duration 15s
    let _ = Example::envoke();
}
```

</br>

#### Loading an environment variable and parsing with a custom parser
```rust
use std::time::Duration;

use envoke::{Envoke, Fill};

#[derive(Fill)]
struct Example {
    #[fill(env = "ENV_INT", parse_fn = parse_time, arg_type = u64)]
    field1: Duration,
}

fn parse_time(time: u64) -> Duration {
    Duration::from_secs(time)
}

fn main() {
    temp_env::with_vars(
        [
            ("ENV_INT"), Some("60"),
        ],
        || {
            // Fills struct field with Duration 60s
            let _ = Example::envoke();
        },
    );
}
```

</br>

#### Nesting multiple structures together
```rust
use envoke::{Envoke, Fill};

#[derive(Fill)]

struct Inner {
    #[fill(env = "ENV_BOOL", default = false)]
    field1: bool,
}

#[derive(Fill)]
struct Outer {
    #[fill(env = "ENV_INT")]
    field1: Duration,

    #[fill(nested)]
    inner: Inner,
}

fn main() {
    temp_env::with_vars(
        [
            ("ENV_INT"), Some("60"),
        ],
        || {
            // Fills outer struct field with `60` and inner field with `false`
            let _ = Example::envoke();
        },
    );
}
```

</br>

#### Disabling prefix and-/or suffix usage
```rust
use envoke::{Envoke, Fill};

#[derive(Fill)]
#[fill(prefix = "PREFIX", suffix = "SUFFIX", delimiter = "_", case = "PacalCase")]
struct Example {
    #[fill(env = "NO_PREFIX")]
    no_prefix: String,

    #[fill(env = "NO_SUFFIX")]
    no_suffix: String,
}

fn main() {
    temp_env::with_vars(
        [
            ("NoPrefixSuffix"), Some("Only environment variable and suffix"),
            ("PrefixNoSuffix"), Some("Only prefix and environment variable"),
        ],
        || {
            // Fills outer struct field with `60` and inner field with `false`
            let _ = Example::envoke();
        },
    );
}
```

</br>

#### Validating a loaded value

Note that if you do not specify where the validation should occur; before or after parsing. It will automatically be called after parsing. Both cases can be seen in the example below

```rust
use envoke::{Envoke, Fill};

fn less_than_ten_opt(amount: &Option<u64>) -> std::result::Result<(), String> {
    if let Some(amount) = amount {
        if *amount > 10 {
            return Err("amount should be less than 10".to_string());
        }
    }

    Ok(())
}

fn more_than_ten_opt(amount: &Option<u64>) -> std::result::Result<(), String> {
    if let Some(amount) = amount {
        if *amount < 10 {
            return Err("amount should be more than 10".to_string());
        }
    }

    Ok(())
}

fn more_than_ten(amount: &u64) -> std::result::Result<(), String> {
    match *amount > 10 {
        true => Ok(()),
        false => Err("amount should be more than 10".to_string()),
    }
}

fn add_ten_opt(amount: Option<u64>) -> Option<u64> {
    amount.and_then(|x| Some(x + 10))
}

fn add_ten(amount: u64) -> u64 {
    amount + 10
}

#[derive(Fill)]
struct Example {
    #[fill(env = "TEST_ENV", parse_fn = add_ten_opt, arg_type = Option<u64>, validate_fn(before = less_than_ten_opt, after = more_than_ten_opt))]
    field1: Option<u64>,

    #[fill(env = "TEST_ENV", parse_fn = add_ten, arg_type = u64, validate_fn = more_than_ten)]
    field2: u64,
}

fn main() {
    temp_env::with_vars(
        [
            ("TEST_ENV", Some("5")),
        ],
        || {
            let _ = Example::envoke();
        },
    );
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