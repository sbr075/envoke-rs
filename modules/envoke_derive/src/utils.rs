use convert_case::Casing;
use quote::quote;
use syn::{Data, Fields, FieldsNamed, PathArguments, Type};

use crate::{attr::ContainerAttributes, Field};

pub fn get_fields(data: Data) -> FieldsNamed {
    match data {
        Data::Struct(s) => match s.fields {
            Fields::Named(fields) => fields,
            _ => panic!("error: fill can only be used for named fields"),
        },
        _ => panic!("error: fill can can only be derived for structs"),
    }
}

pub fn can_be_empty(ty: &Type) -> bool {
    match ty {
        Type::Path(path) => path
            .path
            .segments
            .iter()
            .any(|segment| matches!(segment.arguments, PathArguments::AngleBracketed(_))),
        _ => false,
    }
}

pub fn is_optional(ty: &Type) -> bool {
    match ty {
        Type::Path(path) => path.path.segments[0].ident == "Option",
        _ => false,
    }
}

pub fn load_default(field: &Field) -> proc_macro2::TokenStream {
    let ident = &field.ident;
    let ty = &field.ty;

    let default_expr = if let Some(default_fn) = &field.attrs.default_fn {
        quote! { #default_fn() }
    } else if let Some(default) = &field.attrs.default_t {
        quote! { #default }
    } else if field.attrs.default {
        quote! { <#ty>::default() }
    } else {
        quote! {
            panic!(format!(
                "error: field `{}` has no envs, no defaults, is not optional, and cannot be empty",
                stringify!(#ident)
            ))
        }
    };

    quote! {
        #ident: #default_expr
    }
}

pub fn load_env(attrs: &ContainerAttributes, field: &Field) -> proc_macro2::TokenStream {
    match &field.attrs.envs {
        Some(envs) => {
            let ident = &field.ident;
            let ty = match (&field.attrs.parse_fn.is_some(), &field.attrs.arg_type) {
                (true, None) => {
                    panic!("field attribute `arg_type` is required if `parse_fn` is specified")
                }
                (true, Some(ty)) => ty,
                (false, _) => &field.ty,
            };

            let delim = attrs.get_delimiter();
            let prefix = if !field.attrs.no_prefix {
                format!("{}{delim}", attrs.get_prefix())
            } else {
                String::new()
            };

            let suffix = if !field.attrs.no_suffix {
                format!("{delim}{}", attrs.get_suffix())
            } else {
                String::new()
            };

            let envs: Vec<String> = envs
                .iter()
                .map(|e| format!("{prefix}{e}{suffix}"))
                .map(|env| {
                    attrs
                        .rename_all
                        .as_ref()
                        .map_or(env.clone(), |case| env.to_case(case.into()))
                })
                .collect();

            let use_default = field.attrs.default;
            let requires_delim = can_be_empty(ty) && !is_optional(ty);
            let delim = field.attrs.delimiter.as_deref().unwrap_or(",");

            let base_call = if requires_delim {
                quote! { envoke::Envloader::<#ty>::load_once(&[#(#envs),*], #delim, #use_default) }
            } else {
                quote! { envoke::Envloader::<#ty>::load_once(&[#(#envs),*]) }
            };

            let mut call = if field.attrs.default {
                quote! {
                    {
                        match #base_call {
                            Ok(value) => value,
                            Err(_) => <#ty>::default()
                        }
                    }
                }
            } else if let Some(default_t) = &field.attrs.default_t {
                quote! {
                    {
                        match #base_call {
                            Ok(value) => value,
                            Err(_) => {
                                TryInto::<#ty>::try_into(#default_t)
                                    .map_err(|_| envoke::Error::ParseError(
                                        envoke::ParseError::UnexpectedValueType {
                                            value: #default_t.to_string()
                                        }
                                    ))?
                            }
                        }
                    }
                }
            } else if let Some(default_fn) = &field.attrs.default_fn {
                quote! {
                    {
                        match #base_call {
                            Ok(value) => value,
                            Err(_) => #default_fn()
                        }
                    }
                }
            } else {
                quote! {
                    { #base_call? }
                }
            };

            if let Some(parse_fn) = &field.attrs.parse_fn {
                call = quote! { #parse_fn(#call) }
            }

            quote! {
                #ident: { #call }
            }
        }
        None => load_default(field),
    }
}
