use proc_macro2::TokenStream;
use syn::{spanned::Spanned, Data, DeriveInput};

use crate::errors::Error;

mod common;
mod enums;
mod structs;

pub fn derive_input(input: DeriveInput) -> syn::Result<TokenStream> {
    match &input.data {
        Data::Struct(_) => structs::derive_for(input),
        Data::Enum(_) => enums::derive_for(input),
        _ => Err(Error::UnsupportedTarget.to_syn_error(input.span())),
    }
}
