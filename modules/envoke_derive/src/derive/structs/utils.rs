use proc_macro2::TokenStream;
use quote::quote;

use crate::utils::is_optional;

use super::{
    attrs::{ContainerAttributes, DefaultValue},
    Field,
};

fn generate_default_call(default: &DefaultValue, field: &Field) -> proc_macro2::TokenStream {
    let ident = &field.ident;
    let ident = quote! { #ident }.to_string();

    let ty = &field.ty;
    let ty = quote! { #ty }.to_string();

    let is_optional = is_optional(&field.ty);
    match default {
        DefaultValue::Type(ty) => {
            quote! { <#ty>::default() }
        }
        DefaultValue::Path(path) => {
            let mut call = quote! { #path };
            if is_optional {
                call = quote! { Some(#call) }
            }

            call
        }
        DefaultValue::Lit(lit) => {
            let mut call = quote! {
                #lit.try_into().map_err(|_| envoke::Error::ConvertError {
                    field: #ident.to_string(),
                    ty: #ty.to_string()
                })?
            };
            if is_optional {
                call = quote! { Some(#call) }
            }

            call
        }
        DefaultValue::Call { path, args } => {
            let mut call = quote! { #path(#(#args),*) };
            if is_optional {
                call = quote! { Some(#call) }
            }

            call
        }
    }
}

fn process_call(field: &Field) -> proc_macro2::TokenStream {
    let ident = &field.ident;
    let ident = quote! { #ident }.to_string();
    let mut call = quote! {};

    if let Some(validate_fn) = &field.attrs.validate_fn.before {
        call = quote! {
            #validate_fn(&value).map_err(|e| envoke::Error::ValidationError {
                field: #ident.to_string(),
                err: e.into()
            })?;
        };
    }

    if let Some(parse_fn) = &field.attrs.parse_fn {
        call = quote! {
            #call
            let value = #parse_fn(value);
        }
    }

    if let Some(validate_fn) = &field.attrs.validate_fn.after {
        call = quote! {
            #call
            #validate_fn(&value).map_err(|e| envoke::Error::ValidationError {
                field: #ident.to_string(),
                err: e.into()
            })?;
        };
    }

    call
}

fn generate_env_call(
    envs: &Vec<String>,
    c_attrs: &ContainerAttributes,
    field: &Field,
) -> proc_macro2::TokenStream {
    let ty = match (&field.attrs.parse_fn.is_some(), &field.attrs.arg_type) {
        (true, Some(ty)) => ty,
        _ => &field.ty,
    };

    let envs: Vec<String> = envs
        .iter()
        .map(|env| c_attrs.rename(env.to_owned(), field.attrs.no_prefix, field.attrs.no_suffix))
        .collect();

    let delim = field.attrs.delimiter.as_deref().unwrap_or(",");
    let is_optional = is_optional(ty);
    let base_call = match is_optional {
        true => {
            quote! { envoke::OptEnvloader::<#ty>::load_once(&[#(#envs),*], #delim) }
        }
        false => {
            quote! { envoke::Envloader::<#ty>::load_once(&[#(#envs),*], #delim) }
        }
    };

    let process_call = process_call(field);
    match &field.attrs.default {
        Some(default) => {
            let default_call = generate_default_call(default, field);
            quote! {
                {
                    match #base_call {
                        Ok(value) => {
                            #process_call
                            value
                        },
                        Err(_) => #default_call,
                    }
                }
            }
        }
        None => quote! {
            {
                let value = #base_call?;
                #process_call
                value
            }
        },
    }
}

pub fn generate_field_calls(
    c_attrs: ContainerAttributes,
    fields: Vec<Field>,
) -> syn::Result<Vec<TokenStream>> {
    let mut calls = Vec::new();

    for field in fields {
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

        calls.push(call);
    }

    Ok(calls)
}
