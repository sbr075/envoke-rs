use attrs::{ContainerAttributes, Name, VariantAttributes};
use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{spanned::Spanned, DeriveInput, Ident, Type};
use utils::{generate_variant_calls, get_enum_data};

use crate::errors::Error;

mod attrs;
mod utils;

struct Variant {
    ident: Ident,
    inner_ident: Option<Ident>,
    span: Span,
    attrs: VariantAttributes,
}

impl TryFrom<syn::Variant> for Variant {
    type Error = syn::Error;

    fn try_from(variant: syn::Variant) -> Result<Self, Self::Error> {
        let attrs = VariantAttributes::try_from(&variant)?;

        let inner_ident = match &variant.fields {
            syn::Fields::Unnamed(fields) => {
                let field = fields.unnamed.get(0).unwrap();
                match &field.ty {
                    Type::Path(type_path) => type_path.path.get_ident().cloned(),
                    _ => return Err(Error::UnsupportedVariantType.to_syn_error(variant.span())),
                }
            }
            syn::Fields::Unit => None,
            _ => return Err(Error::UnsupportedEnumType.to_syn_error(variant.span())),
        };

        Ok(Self {
            ident: variant.ident.clone(),
            inner_ident,
            span: variant.span(),
            attrs,
        })
    }
}

impl Variant {
    fn get_names(&self) -> Vec<Name> {
        let mut names = self.attrs.aliases.clone().unwrap_or_default();

        // Check if field name was renamed, else we use the original field name
        match &self.attrs.rename {
            Some(rename) => names.insert(0, rename.clone()),
            None => {
                let ident = &self.ident;
                let value = quote! { #ident }.to_string();

                names.insert(0, Name { value, span: None });
            }
        }

        names
    }
}

pub fn derive_for(input: DeriveInput) -> syn::Result<TokenStream> {
    let enum_name = &input.ident;
    let (impl_generics, type_generics, where_clause) = input.generics.split_for_impl();

    let c_attrs = ContainerAttributes::try_from(&input)?;
    let envs = c_attrs.get_envs();

    let value_call =
        quote! { envoke::Envloader::<String>::load_once(&[#(#envs),*], ",", dotenv.as_ref()) };

    let enum_data = get_enum_data(input.data)?;
    let variants: Vec<Variant> = enum_data
        .variants
        .into_iter()
        .map(Variant::try_from)
        .collect::<syn::Result<_>>()?;

    // Create the dotenv call here but it will be used when generating the variant
    // calls below
    let dotenv_call = match &c_attrs.dotenv {
        Some(dotenv) => {
            quote! {
                let dotenv = Some(load_dotenv(#dotenv)?);
            }
        }
        // Not the real type but it just needs a type
        None => quote! {
            let dotenv: Option<std::collections::HashMap<String, String>> = None;
        },
    };

    let (calls, default_call) = generate_variant_calls(enum_name, variants, c_attrs)?;

    let value_call = match default_call {
        Some(default) => quote! {
            let value = match #value_call {
                Ok(value) => value,
                Err(_) => return Ok(#default)
            };

            let mut found = None;
            #(#calls);*

            match found {
                Some(value) => Ok(value),
                None => Ok(#default)
            }
        },
        None => quote! {
            let value = #value_call?;

            let mut found = None;
            #(#calls);*

            match found {
                Some(value) => Ok(value),
                None => Err(envoke::Error::EnumError(envoke::EnumError::NotFound))
            }
        },
    };

    let expanded = quote! {
        impl #impl_generics envoke::Envoke for #enum_name #type_generics #where_clause {
            fn try_envoke() -> envoke::Result<#enum_name #type_generics> {
                use envoke::{Envloader, load_dotenv};

                #dotenv_call

                #value_call
            }
        }
    };

    Ok(expanded)
}
