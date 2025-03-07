use attrs::{ContainerAttributes, Name, VariantAttributes};
use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{spanned::Spanned, Data, DataEnum, DeriveInput, Ident, Type};

use crate::errors::Error;

mod attrs;

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
                    _ => return Err(Error::UnsupportedEnumType.to_syn_error(variant.span())),
                }
            }
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

                names.push(Name { value, span: None });
            }
        }

        names
    }
}

fn get_enum_data(data: Data) -> syn::Result<DataEnum> {
    match data {
        Data::Enum(data_enum) => Ok(data_enum),
        _ => unreachable!(),
    }
}

fn generate_variant_calls(
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

            default_call = Some(quote! { #ident::try_envoke()? });
        }
    }

    Ok((calls, default_call))
}

pub fn derive_for(input: DeriveInput) -> syn::Result<TokenStream> {
    let enum_name = &input.ident;
    let (impl_generics, type_generics, where_clause) = input.generics.split_for_impl();

    let c_attrs = ContainerAttributes::try_from(&input)?;
    let envs = c_attrs.get_envs();

    let value_call = quote! { envoke::Envloader::<String>::load_once(&[#(#envs),*], ",") };

    let enum_data = get_enum_data(input.data)?;
    let variants: Vec<Variant> = enum_data
        .variants
        .into_iter()
        .map(Variant::try_from)
        .collect::<syn::Result<_>>()?;

    let (calls, default_call) = generate_variant_calls(enum_name, variants, c_attrs)?;

    let unwrap_call = match default_call {
        Some(default) => quote! {
            match found {
                Some(value) => Ok(value),
                None => #default
            }
        },
        None => quote! {
            match found {
                Some(value) => Ok(value),
                None => Err(envoke::Error::EnumError(EnumError::NotFound))
            }
        },
    };

    let expanded = quote! {
        impl #impl_generics envoke::Envoke for #enum_name #type_generics #where_clause {
            fn try_envoke() -> envoke::Result<#enum_name #type_generics> {
                use envoke::{Envloader, EnumError};

                let value = #value_call?;
                let mut found = None;

                #(#calls);*

                #unwrap_call
            }
        }
    };

    Ok(expanded)
}
