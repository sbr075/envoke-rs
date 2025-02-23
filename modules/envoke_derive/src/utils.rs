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

pub fn default_call(field: &Field) -> proc_macro2::TokenStream {
    match &field.attrs.default {
        Some(default) => match default {
            crate::attr::DefaultValue::Type(ty) => {
                quote! { <#ty>::default() }
            }
            crate::attr::DefaultValue::Path(path) => {
                quote! { #path }
            }
            crate::attr::DefaultValue::Lit(lit) => {
                quote! { #lit }
            }
            crate::attr::DefaultValue::Call { path, args } => {
                quote! { #path(#(#args),*) }
            }
        },
        None => quote! { panic!("fatal error occurred") },
    }
}

pub fn env_call(attrs: &ContainerAttributes, field: &Field) -> proc_macro2::TokenStream {
    if let Some(envs) = &field.attrs.envs {
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

        let requires_delim = can_be_empty(ty) && !is_optional(ty);
        let delim = field.attrs.delimiter.as_deref().unwrap_or(",");

        let base_call = if requires_delim {
            quote! { envoke::Envloader::<#ty>::load_once(&[#(#envs),*], #delim) }
        } else {
            quote! { envoke::Envloader::<#ty>::load_once(&[#(#envs),*]) }
        };

        let mut call = match field.attrs.default.is_some() {
            true => {
                let default_call = default_call(field);
                quote! {
                    {
                        match #base_call {
                            Ok(value) => value,
                            Err(_) => #default_call,
                        }
                    }
                }
            }
            false => {
                quote! {
                    { #base_call? }
                }
            }
        };

        if let Some(parse_fn) = &field.attrs.parse_fn {
            call = quote! { #parse_fn(#call) }
        }

        call
    } else {
        quote! {
            panic!("fatal error occurred")
        }
    }
}
