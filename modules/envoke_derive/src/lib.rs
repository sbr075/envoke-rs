use attr::{ContainerAttributes, FieldAttributes};
use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Type};
use utils::{get_fields, load_default, load_env};

mod attr;
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
        let ident = field.ident;

        let name = quote! { #ident }.to_string();
        let attrs = FieldAttributes::parse_attrs(&name, &field.attrs)?;

        Ok(Self {
            ident,
            ty: field.ty,
            attrs,
        })
    }
}

#[proc_macro_derive(Fill, attributes(fill))]
pub fn derive(input: TokenStream) -> TokenStream {
    let input: DeriveInput = parse_macro_input!(input);
    let struct_name = &input.ident;

    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let attrs = match ContainerAttributes::parse_attrs(&input.attrs) {
        Ok(attrs) => attrs,
        Err(e) => panic!("error: failed to parse container attributes for `{struct_name}`: {e}"),
    };

    let mut field_assignments = Vec::new();

    let fields = get_fields(input.data);
    for field in fields.named {
        let field = match Field::try_from(field) {
            Ok(field) => field,
            Err(e) => panic!("error: {e}"),
        };

        let ident = &field.ident;
        let ty = &field.ty;
        if field.attrs.is_empty() {
            let field_name = quote! { #ident }.to_string();
            panic!(
                "atleast one of the following field attributes is required for `{field_name}`: \
                 `env`, `default_t`, `default_t`, `default_fn`, or `nested`"
            )
        }

        let tokens = if field.attrs.nested {
            quote! {
                #ident: <#ty as envoke::Envoke>::try_envoke()?
            }
        } else {
            match &field.attrs.envs.is_some() {
                true => load_env(&attrs, &field),
                false => load_default(&field),
            }
        };

        field_assignments.push(tokens);
    }

    let expanded = quote! {
        impl #impl_generics envoke::Envoke for #struct_name #ty_generics #where_clause {
            fn try_envoke() -> envoke::Result<#struct_name #ty_generics> {
                use envoke::Envload;

                Ok(#struct_name {
                    #(#field_assignments),*
                })
            }
        }
    };

    TokenStream::from(expanded)
}
