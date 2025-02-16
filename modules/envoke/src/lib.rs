mod errors;
mod load;

pub use errors::{Error, ParseError, Result, RetrieveError};
pub use load::{Envload, Envloader};

pub use envoke_derive::Fill;

pub trait Envoke: Sized {
    /// Creates an instance of `Self` by loading values from environment
    /// variables.
    ///
    /// This method **will panic** if any required environment variables are
    /// missing or cannot be parsed into the expected types.
    ///
    /// If fallible behavior is needed, use [`try_envoke`] instead.
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
