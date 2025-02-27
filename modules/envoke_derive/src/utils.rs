use convert_case::Casing;
use quote::quote;
use syn::{Data, Fields, FieldsNamed, GenericArgument, PathArguments, Type};

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

pub fn is_optional(ty: &Type) -> bool {
    match ty {
        Type::Path(path) => path.path.segments[0].ident == "Option",
        _ => false,
    }
}

fn get_inner_types(ty: &Type) -> Option<Vec<&Type>> {
    match ty {
        Type::Path(path) => match path.path.segments.get(0) {
            Some(segment) => match &segment.arguments {
                PathArguments::AngleBracketed(args) => {
                    let inners = args
                        .args
                        .iter()
                        .filter_map(|e| match e {
                            GenericArgument::Type(ty) => Some(ty),
                            _ => None,
                        })
                        .collect();

                    Some(inners)
                }
                _ => None,
            },
            None => None,
        },
        _ => None,
    }
}

pub fn default_call(field: &Field) -> proc_macro2::TokenStream {
    let ident = &field.ident;
    let ident = quote! { #ident }.to_string();

    let ty = &field.ty;
    let ty = quote! { #ty }.to_string();

    let is_optional = is_optional(&field.ty);
    match &field.attrs.default {
        Some(default) => match default {
            crate::attr::DefaultValue::Type(ty) => {
                quote! { <#ty>::default() }
            }
            crate::attr::DefaultValue::Path(path) => {
                let mut call = quote! { #path };
                if is_optional {
                    call = quote! { Some(#call) }
                }

                call
            }
            crate::attr::DefaultValue::Lit(lit) => {
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
            crate::attr::DefaultValue::Call { path, args } => {
                let mut call = quote! { #path(#(#args),*) };
                if is_optional {
                    call = quote! { Some(#call) }
                }

                call
            }
        },
        None => quote! { panic!("fatal error occurred") },
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

        let delim = field.attrs.delimiter.as_deref().unwrap_or(",");
        let is_optional = is_optional(ty);
        let base_call = match is_optional {
            true => {
                let inner_types = get_inner_types(ty).unwrap();
                quote! { <envoke::Envloader<#ty> as FromSingleOpt<(#(#inner_types),*)>>::load_once(&[#(#envs),*], #delim) }
            }
            false => {
                quote! { envoke::Envloader::<#ty>::load_once(&[#(#envs),*], #delim) }
            }
        };

        let process_call = process_call(field);
        match field.attrs.default.is_some() {
            true => {
                let default_call = default_call(field);
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
            false => {
                quote! {
                    {
                        let value = #base_call?;
                        #process_call
                        value
                    }
                }
            }
        }
    } else {
        quote! {
            panic!("fatal error occurred")
        }
    }
}
