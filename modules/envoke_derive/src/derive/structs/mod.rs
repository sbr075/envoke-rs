use attrs::{ContainerAttributes, FieldAttributes};
use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{spanned::Spanned, Data, DeriveInput, Fields, FieldsNamed, Ident, Type};
use utils::{generate_default_call, generate_env_call};

use crate::errors::Error;

mod attrs;
mod utils;

struct Field {
    ident: Option<Ident>,
    ty: Type,
    attrs: FieldAttributes,
}

impl TryFrom<syn::Field> for Field {
    type Error = syn::Error;

    fn try_from(field: syn::Field) -> Result<Self, Self::Error> {
        let attrs = FieldAttributes::try_from(&field)?;
        Ok(Self {
            ident: field.ident,
            ty: field.ty,
            attrs,
        })
    }
}

fn get_struct_data(span: Span, data: Data) -> syn::Result<FieldsNamed> {
    match data {
        Data::Struct(data_struct) => match data_struct.fields {
            Fields::Named(fields) => Ok(fields),
            _ => return Err(Error::UnsupportedStructType.to_syn_error(span)),
        },
        _ => unreachable!(),
    }
}

pub fn derive_for(input: DeriveInput) -> syn::Result<TokenStream> {
    let (impl_generics, type_generics, where_clause) = input.generics.split_for_impl();

    let c_attrs = ContainerAttributes::try_from(&input)?;

    let struct_name = &input.ident;
    let struct_data = get_struct_data(input.span(), input.data)?;

    let mut field_assignments = Vec::new();
    for field in struct_data.named {
        // Extract only what we need
        let field = Field::try_from(field)?;

        let ident = &field.ident;
        let ty = &field.ty;

        let value_call = if field.attrs.is_nested {
            quote! {
                <#ty as envoke::Envoke>::try_envoke()?
            }
        } else if let Some(envs) = &field.attrs.envs {
            generate_env_call(&envs, &c_attrs, &field)
        } else if let Some(default) = &field.attrs.default {
            generate_default_call(&default, &field)
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
        impl #impl_generics envoke::Envoke for #struct_name #type_generics #where_clause {
            fn try_envoke() -> envoke::Result<#struct_name #type_generics> {
                use envoke::{Envloader, OptEnvloader, FromMap, FromMapOpt, FromSet, FromSetOpt};

                Ok(#struct_name {
                    #(#field_assignments),*
                })
            }
        }
    };

    Ok(expanded)
}
