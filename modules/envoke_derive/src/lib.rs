//! # envoke_derive
//!
//! Derive macro for `envoke`. Learn more [here!](https://docs.rs/envoke)
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

use derive::derive_input;
use proc_macro::TokenStream;

mod derive;
mod errors;
mod utils;

#[doc(hidden)]
#[proc_macro_derive(Fill, attributes(fill))]
pub fn derive(input: TokenStream) -> TokenStream {
    let input: syn::DeriveInput = syn::parse_macro_input!(input);

    let tokens = match derive_input(input) {
        Ok(tokens) => tokens,
        Err(e) => return e.to_compile_error().into(),
    };

    TokenStream::from(tokens)
}
