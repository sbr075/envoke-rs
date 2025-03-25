use attrs::{ContainerAttributes, FieldAttributes};
use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{spanned::Spanned, Data, DeriveInput, Fields, FieldsNamed, Ident, Type};
use utils::generate_field_calls;

use crate::errors::Error;

mod attrs;
mod utils;

#[derive(Debug)]
pub struct Field {
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
    let fields: Vec<Field> = struct_data
        .named
        .into_iter()
        .map(Field::try_from)
        .collect::<syn::Result<_>>()?;

    // Create the dotenv call here but it will be used when generating the field
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

    let field_calls = generate_field_calls(c_attrs, fields)?;

    let expanded = quote! {
        impl #impl_generics envoke::Envoke for #struct_name #type_generics #where_clause {
            fn try_envoke() -> envoke::Result<#struct_name #type_generics> {
                use envoke::{Envloader, OptEnvloader, FromMap, FromMapOpt, FromSetOpt, FromSet, load_dotenv};

                #dotenv_call

                Ok(#struct_name {
                    #(#field_calls),*
                })
            }
        }
    };

    Ok(expanded)
}
