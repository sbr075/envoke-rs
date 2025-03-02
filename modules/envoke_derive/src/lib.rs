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

use attr::{ContainerAttributes, FieldAttributes};
use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::quote;
use syn::{parse_macro_input, spanned::Spanned, DeriveInput, Type};
use utils::{default_call, env_call, get_fields};

mod attr;
mod errors;
mod utils;

#[derive(Debug)]
struct Field {
    ident: Option<Ident>,
    ty: Type,
    attrs: FieldAttributes,
}

impl TryFrom<syn::Field> for Field {
    type Error = syn::Error;

    fn try_from(field: syn::Field) -> Result<Self, Self::Error> {
        let attrs = FieldAttributes::parse_attrs(&field, &field.attrs)?;

        Ok(Self {
            ident: field.ident,
            ty: field.ty,
            attrs,
        })
    }
}

#[doc(hidden)]
#[proc_macro_derive(Fill, attributes(fill))]
pub fn derive(input: TokenStream) -> TokenStream {
    let input: DeriveInput = parse_macro_input!(input);
    let struct_name = &input.ident;

    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let attrs = match ContainerAttributes::parse_attrs(&input.attrs) {
        Ok(attrs) => attrs,
        Err(e) => return e.to_compile_error().into(),
    };

    let fields = match get_fields(&input.span(), input.data) {
        Ok(fields) => fields,
        Err(e) => return e.to_compile_error().into(),
    };

    let mut field_assignments = Vec::new();

    for field in fields.named {
        let field = match Field::try_from(field) {
            Ok(field) => field,
            Err(e) => return e.to_compile_error().into(),
        };

        let ident = &field.ident;
        let ty = &field.ty;

        let value_call = if field.attrs.is_nested {
            quote! {
                <#ty as envoke::Envoke>::try_envoke()?
            }
        } else if let Some(envs) = &field.attrs.envs {
            env_call(&envs, &attrs, &field)
        } else if let Some(default) = &field.attrs.default {
            default_call(&default, &field)
        } else {
            // Caught by another check
            unreachable!()
        };

        let call = quote! {
            #ident: #value_call
        };

        field_assignments.push(call);
    }

    let expanded = quote! {
        impl #impl_generics envoke::Envoke for #struct_name #ty_generics #where_clause {
            fn try_envoke() -> envoke::Result<#struct_name #ty_generics> {
                use envoke::{Envloader, OptEnvloader, FromMap, FromMapOpt, FromSet, FromSetOpt};

                Ok(#struct_name {
                    #(#field_assignments),*
                })
            }
        }
    };

    TokenStream::from(expanded)
}
