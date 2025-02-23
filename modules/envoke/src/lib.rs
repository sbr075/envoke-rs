//! # envoke
//!
//! Useful links
//! * [Github](https://github.com/sbr075/envoke-rs)
//! * [CHANGELOG](https://github.com/sbr075/envoke-rs/blob/main/CHANGELOG.md)
//!
//! ## Attributes
//!
//! Container attributes apply to the whole struct influencing the behavior of
//! the entire structure. Field attributes are applied to individual fields
//! within struct, providing fine-grained control over how each field is
//! processed. Both types of attributes allow for customizing how data is
//! loaded, transformed, or handled.
//!
//! </br>
//!
//! ### Container
//!
//! Below are the current implemented container attributes. This list will be
//! updated as more are added or changed.
//!
//! | Attribute    | Default | Description                                                                                                                                                                                                                                                                                                                                                                                  |
//! | ------------ | ------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
//! | `prefix`     | None    | Set a custom prefix which will be prepended infront of environment variables before fetching                                                                                                                                                                                                                                                                                                 |
//! | `suffix`     | None    | Set a custom prefix which will be appended infront of environment variables before fetching                                                                                                                                                                                                                                                                                                  |
//! | `delimiter`  | None    | Set a customer delimiter used for separated prefix, environment variable, and suffix. **NB!** If you are using the `rename_all` attribute as well it will take priority over the delimiter. It can still be useful to include the delimiter to ensure the prefix, environment variable, and suffix are separated before renaming occurs otherwise they will be interpreted as a single word! |
//! | `rename_all` | None    | Rename all environment variables to a different naming case. See [name cases](#name-cases) for a full list and description of the different options.                                                                                                                                                                                                                                         |
//!
//! </br>
//!
//! #### Naming cases
//!
//! | Case                 | Value                  | Description                                                                                                        |
//! | -------------------- | ---------------------- | ------------------------------------------------------------------------------------------------------------------ |
//! | Lower case           | `lowercase` or `lower` | Converts all characters to lowercase and removes binding characters                                                |
//! | Upper case           | `UPPERCASE` or `UPPER` | Converts all characters to uppercase and removes binding characters                                                |
//! | Pascal case          | `PascalCase`           | Capitalizes the first letter of each word and removes binding                                                      |
//! | Camel case           | `camelCase`            | Lowercases the first letter but capitalizes the first letter of subsequent words while removing binding characters |
//! | Snake case           | `snake_case`           | Converts names to lowercase and uses underscores `_` to separate words                                             |
//! | Screaming snake case | `SCREAMING_SNAKE_CASE` | Converts names to uppercase and uses underscores `_` to separate words                                             |
//! | Kebab case           | `kebab-case`           | Converts names to lowercase and uses hyphens `-` to separate words                                                 |
//! | Screaming kebab case | `SCREAMING-KEBAB-CASE` | Converts names to uppercase and uses hyphens `-` to separate words                                                 |
//!
//! </br>
//!
//! ### Field
//!
//! Below are the current implemented field attributes. This list will be
//! updated as more are added or changed.
//!
//! | Attribute   | Default   | Description                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
//! | ----------- | --------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
//! | `env`       | None      | Environment variable name to load the field value from. Can be chained multiple times to allow for fallbacks. The macro follows a first come, first serve basis meaning it attempts to load the variables in the order they are listed. Once an value is found it will try to parse it into the specified type. If it fails it will return an error and wont try the remaining ones in the list. This behavior might change in the future. Optionally, you can supply your own parsing function. See `parse_fn` for more information! |
//! | `default`   | None      | Use the default value if the environment variable is not found. Optionally to statically assign a value to the field `env` can be omitted.                                                                                                                                                                                                                                                                                                                                                                                            |
//! | `parse_fn`  | None      | Set a custom parsing function for parsing the retrieved value before assigning it to the field. This can be useful when the fields type does not implement the `FromStr` trait. Requires `arg_type` to be set                                                                                                                                                                                                                                                                                                                         |
//! | `arg_type`  | None      | Specify the argument type which the `parse_fn` function requires. As I don't know if it is possible to find the type automatically this argument is required such that the environment variable value can be parsed into the expected type first before being set as the argument in the function call.                                                                                                                                                                                                                               |
//! | `delimiter` | Comma (,) | Used when parsing environment variable which is a stringified map or set. The delimiter specifies the boundary between values.                                                                                                                                                                                                                                                                                                                                                                                                        |
//! | `no_prefix` | False     | Disable adding the global prefix to this environment variable. This will also remove the delimiter that wouldn't normally be between the environment variable and prefix                                                                                                                                                                                                                                                                                                                                                              |
//! | `no_suffix` | False     | Disable adding the global suffix to this environment variable. This will also remove the delimiter that wouldn't normally be between the environment variable and suffix                                                                                                                                                                                                                                                                                                                                                              |
//! | `nested`    | False     | Indicate that the field is a struct. Required when the field type is another struct                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
//!
//! </br>
//!
//! ## Examples
//!
//! </br>
//!
//! The simplest example of loading environment variable into a struct is
//!
//! ```rust
//! use envoke::Envoke;
//!
//! #[derive(Fill)]
//! #[fill(rename_all = "UPPERCASE")]
//! struct Environment {
//!     #[fill(env)]
//!     field: String,
//! }
//!
//! fn main() {
//!     let env = Environment::envoke();
//! }
//! ```
//!
//! </br>
//!
//! It can also be used similar to the standard trait [Default](https://doc.rust-lang.org/std/default/trait.Default.html)
//!
//! ```rust
//! use envoke::Envoke;
//!
//! #[derive(Fill)]
//! #[fill(rename_all = "UPPERCASE")]
//! struct Environment {
//!     #[fill(env)]
//!     field1: String,
//!
//!     #[fill(env)]
//!     field2: String,
//! }
//!
//! fn main() {
//!     let env = Environment {
//!         field1: "Hello, User!".to_string(),
//!         ..Envoke::envoke()
//!     };
//! }
//! ```
//!
//! </br>
//!
//! Multiple environment variable names can be chained in a row to add fallbacks
//!
//! ```rust
//! use envoke::Envoke;
//!
//! #[derive(Fill)]
//! #[fill(rename_all = "UPPERCASE")]
//! struct Environment {
//!     #[fill(env, env = "FALLBACK1", env = "FALLBACK2")]
//!     field: String,
//! }
//!
//! fn main() {
//!     let env = Environment::envoke();
//! }
//! ```
//!
//! The environment variables will be loaded in the order they are defined. One
//! the first successful load it will be parsed and set as the fields value. If
//! the parsing fails the macro will return an error and not retry the other
//! variables. This behavior might change in the future!
//!
//! Note that just supplying `env` means just use the field name as the
//! environment variable name and is not required.
//!
//! </br>
//!
//! To use enums we recommend using the [strum](https://crates.io/crates/strum) crate as such
//!
//! ```rust
//! use envoke::Envoke;
//!
//! #[derive(strum::EnumString)]
//! #[strum(serialize_all = "lowercase")]
//! enum Level {
//!     Info,
//!     Warn,
//!     Error,
//! }
//!
//! #[derive(Fill)]
//! #[fill(rename_all = "UPPERCASE")]
//! struct Environment {
//!     #[fill(env)]
//!     level: Level,
//! }
//!
//! fn main() {
//!     let env = Environment::envoke();
//! }
//! ```
//!
//! </br>
//!
//! Behind the scenes it is required for the type to bet set implements the [FromStr](https://doc.rust-lang.org/std/str/trait.FromStr.html) trait.
//! Although this can be omitted by implementing a custom parsing function
//!
//! ```rust
//! use std::time::Duration;
//!
//! use envoke::Envoke;
//!
//! fn to_time(sec: u64) -> Duration {
//!     Duration::from_secs(sec)
//! }
//!
//! #[derive(Fill)]
//! #[fill(rename_all = "UPPERCASE")]
//! struct Environment {
//!     #[fill(env, parse_fn = to_time, arg_type = u64)]
//!     dur: Duration,
//! }
//!
//! fn main() {
//!     let env = Environment::envoke();
//! }
//! ```
//!
//! </br>
//!
//! Default fallbacks can also be added, or used alone
//!
//! ```rust
//! use envoke::Envoke;
//!
//! #[derive(Fill)]
//! #[fill(rename_all = "UPPERCASE")]
//! struct Environment {
//!     #[fill(env, default)]
//!     field: String,
//! }
//!
//! fn main() {
//!     let env = Environment::envoke();
//! }
//! ```
//!
//! Default can be used alone which will load the field types default value,
//! supports direct value assignment, and a function call which returns the
//! fields type
//!
//! </br>
//!
//! There are more container-/field attributes not mentioned here so take a look
//! at them! They are designed to be used together for both simple and complex
//! environment loading. If there are any attributes you feel are missing open
//! an issue on our github page and we'll take a look at is as soon as possible!
//!
//! </br>
//!
//! #### License
//!
//! <sup>
//! Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
//! 2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
//! </sup>
//!
//! </br>
//!
//! <sub>
//! Unless you explicitly state otherwise, any contribution intentionally
//! submitted for inclusion in this crate by you, as defined in the Apache-2.0
//! license, shall be dual licensed as above, without any additional terms or
//! conditions. </sub>

mod errors;
mod load;

#[doc(hidden)]
pub use errors::{Error, ParseError, Result, RetrieveError};

#[doc(hidden)]
pub use load::{Envload, Envloader};

#[doc(hidden)]
pub use envoke_derive::Fill;

pub trait Envoke: Sized {
    /// Creates an instance of `Self` by loading values from environment
    /// variables.
    ///
    /// This method **will panic** if any required environment variables are
    /// missing or cannot be parsed into the expected types.
    ///
    /// If fallible behavior is needed, use [`Envoke::try_envoke`] instead.
    ///
    /// # Panics
    /// Panics if any required environment variables are missing or invalid.
    ///
    /// # Examples
    ///
    /// ```
    /// use envload::Envoke;
    ///
    /// #[derive(Envoke)]
    /// struct Config {
    ///     #[fill(env = "TEST_ENV")]
    ///     key: String,
    /// }
    ///
    /// let config = Config::envoke(); // Panics if `key` is missing
    /// ```
    fn envoke() -> Self {
        Envoke::try_envoke().unwrap()
    }

    /// Attempts to create an instance of `Self` by loading values from
    /// environment variables.
    ///
    /// This method returns an error if any required environment variables are
    /// missing or cannot be parsed into the expected types.
    ///
    /// # Errors
    /// Returns an error if environment variables are missing or cannot be
    /// parsed.
    ///
    /// # Examples
    ///
    /// ```
    /// use envload::Envoke;
    ///
    /// #[derive(Envoke)]
    /// struct Config {
    ///     #[fill(env = "TEST_ENV")]
    ///     key: String,
    /// }
    ///
    /// match Config::try_envoke() {
    ///     Ok(config) => println!("Config loaded successfully"),
    ///     Err(err) => eprintln!("Failed to load config: {}", err),
    /// }
    /// ```
    fn try_envoke() -> Result<Self>;
}
