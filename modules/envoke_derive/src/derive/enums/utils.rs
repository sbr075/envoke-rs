use proc_macro2::TokenStream;
use quote::quote;
use syn::{Data, DataEnum, Ident};

use crate::errors::Error;

use super::{attrs::ContainerAttributes, Variant};

pub fn get_enum_data(data: Data) -> syn::Result<DataEnum> {
    match data {
        Data::Enum(data_enum) => Ok(data_enum),
        _ => unreachable!(),
    }
}

pub fn generate_variant_calls(
    enum_name: &Ident,
    variants: Vec<Variant>,
    c_attrs: ContainerAttributes,
) -> syn::Result<(Vec<TokenStream>, Option<TokenStream>)> {
    let mut calls = Vec::new();
    let mut default_call = None;

    let mut existing_names = Vec::new();
    for variant in variants {
        let ident = &variant.ident;
        let inner_ident = &variant.inner_ident;

        let names = variant.get_names();

        // Check for duplicate names
        let mut renamed = Vec::new();
        for name in names {
            let new_name = c_attrs.rename(
                name.value.clone(),
                variant.attrs.no_prefix,
                variant.attrs.no_suffix,
            );

            if existing_names.contains(&new_name) {
                return Err(Error::already_used(format!("name::{}", name.value))
                    .to_syn_error(name.span.unwrap_or(variant.span)));
            }

            existing_names.push(name.value);
            renamed.push(new_name);
        }

        // Generate match call
        let call = quote! {
            if [#(#renamed),*].iter().any(|n| value.eq(n)) {
                found = Some(#enum_name::#ident(#inner_ident::try_envoke()?))
            }
        };
        calls.push(call);

        // Assign default if applicable
        if let Some(default) = variant.attrs.default {
            if default_call.is_some() {
                return Err(Error::duplicate_attribute("default").to_syn_error(default.span));
            }

            default_call = Some(quote! { #enum_name::#ident(#inner_ident::try_envoke()?) });
        }
    }

    Ok((calls, default_call))
}
