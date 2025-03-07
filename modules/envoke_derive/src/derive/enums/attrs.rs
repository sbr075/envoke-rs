use convert_case::{Case as ConvertCase, Casing};
use proc_macro2::Span;
use quote::quote;
use syn::{meta::ParseNestedMeta, spanned::Spanned, DeriveInput};

use crate::{derive::common::Case, errors::Error, utils::find_closest_match};

#[derive(Debug, Default)]
pub struct ContainerAttributes {
    // Envvars to look for
    pub envs: Option<Vec<String>>,

    // Change case of names
    pub rename_all: Option<Case>,

    // Prefix to put infront of all names
    pub prefix: Option<String>,

    // Suffix to put after all names
    pub suffix: Option<String>,

    // Delimiter used to separate prefix, name, and suffix
    pub delimiter: Option<String>,
}

impl ContainerAttributes {
    const VARIANTS: &[&str] = &["env", "rename_all", "prefix", "suffix", "delimiter"];

    fn add_env(&mut self, input: &DeriveInput, meta: ParseNestedMeta) -> syn::Result<()> {
        // Allows the user to specify both
        // 1. `#[fill(env)]` - Uses the field name as environment variable
        // 2. `#[fill(env = "env")]` - Uses `env` as the environment variable
        let env = match meta.input.peek(syn::Token![=]) {
            true => {
                let str: syn::LitStr = meta.value()?.parse()?;
                let env = str.value();
                if env.is_empty() {
                    return Err(Error::invalid_attribute("env", "attribute cannot be empty")
                        .to_syn_error(meta.path.span()));
                }

                if self.envs.as_ref().is_some_and(|e| e.contains(&env)) {
                    return Err(Error::duplicate_attribute(format!("env::{env}"))
                        .to_syn_error(meta.path.span()));
                }

                env
            }
            false => {
                let ident = &input.ident;
                let env = quote! { #ident }.to_string();

                if self.envs.as_ref().is_some_and(|e| e.contains(&env)) {
                    return Err(Error::duplicate_attribute(format!("env::{env}"))
                        .to_syn_error(meta.path.span()));
                }

                env
            }
        };

        self.envs.get_or_insert(Vec::new()).push(env);
        Ok(())
    }

    fn set_rename_all(&mut self, meta: ParseNestedMeta) -> syn::Result<()> {
        if self.rename_all.is_some() {
            return Err(Error::duplicate_attribute("rename_all").to_syn_error(meta.path.span()));
        }

        let case: Case = meta.value()?.parse()?;
        self.rename_all = Some(case);
        Ok(())
    }

    fn set_prefix(&mut self, meta: ParseNestedMeta) -> syn::Result<()> {
        if self.prefix.is_some() {
            return Err(Error::duplicate_attribute("prefix").to_syn_error(meta.path.span().span()));
        }

        let prefix: syn::LitStr = meta.value()?.parse()?;
        self.prefix = Some(prefix.value());
        Ok(())
    }

    fn set_suffix(&mut self, meta: ParseNestedMeta) -> syn::Result<()> {
        if self.suffix.is_some() {
            return Err(Error::duplicate_attribute("suffix").to_syn_error(meta.path.span().span()));
        }

        let suffix: syn::LitStr = meta.value()?.parse()?;
        self.suffix = Some(suffix.value());
        Ok(())
    }

    fn set_delimiter(&mut self, meta: ParseNestedMeta) -> syn::Result<()> {
        if self.delimiter.is_some() {
            return Err(
                Error::duplicate_attribute("delimiter").to_syn_error(meta.path.span().span())
            );
        }

        let delimiter: syn::LitStr = meta.value()?.parse()?;
        self.delimiter = Some(delimiter.value());
        Ok(())
    }

    fn get_prefix(&self) -> &str {
        self.prefix.as_deref().unwrap_or_default()
    }

    fn get_suffix(&self) -> &str {
        self.suffix.as_deref().unwrap_or_default()
    }

    fn get_delimiter(&self) -> &str {
        self.delimiter.as_deref().unwrap_or_default()
    }

    pub fn rename(&self, original: String, no_prefix: bool, no_suffix: bool) -> String {
        let delim = self.get_delimiter();
        let prefix = if !no_prefix {
            format!("{}{delim}", self.get_prefix())
        } else {
            String::new()
        };

        let suffix = if !no_suffix {
            format!("{delim}{}", self.get_suffix())
        } else {
            String::new()
        };

        let renamed = format!("{prefix}{original}{suffix}");

        if let Some(case) = &self.rename_all {
            let convert_case = ConvertCase::from(case);
            renamed.to_case(convert_case)
        } else {
            renamed
        }
    }

    pub fn get_envs(&self) -> Vec<String> {
        self.envs
            .clone()
            .unwrap()
            .into_iter()
            .map(|e| self.rename(e, false, false))
            .collect()
    }
}

impl TryFrom<&DeriveInput> for ContainerAttributes {
    type Error = syn::Error;

    fn try_from(input: &DeriveInput) -> Result<Self, Self::Error> {
        let mut ca = ContainerAttributes::default();
        for attr in &input.attrs {
            if !attr.path().is_ident("fill") {
                continue;
            }

            attr.parse_nested_meta(|meta| {
                let ident = meta.path.get_ident();
                let ident = quote! { #ident }.to_string();

                match ident.as_ref() {
                    "env" => ca.add_env(&input, meta),
                    "rename_all" => ca.set_rename_all(meta),
                    "prefix" => ca.set_prefix(meta),
                    "suffix" => ca.set_suffix(meta),
                    "delimiter" => ca.set_delimiter(meta),
                    _ => {
                        let closest_match = find_closest_match(&ident, Self::VARIANTS);
                        Err(Error::unexpected_attribute(ident, closest_match)
                            .to_syn_error(meta.path.span()))
                    }
                }?;

                Ok(())
            })?;
        }

        // Add container name as env if no env given
        if ca.envs.is_none() {
            let ident = &input.ident;
            let env = quote! { #ident }.to_string();

            ca.envs.get_or_insert(Vec::new()).push(env);
        }

        Ok(ca)
    }
}

#[derive(Debug)]
pub struct Default {
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct Name {
    pub value: String,
    pub span: Option<Span>,
}

#[derive(Debug, Default)]
pub struct VariantAttributes {
    // Rename overwrites the normal enum field name
    pub rename: Option<Name>,

    // Aliases are included along the original/renamed field
    pub aliases: Option<Vec<Name>>,

    // Opt out of using prefix
    pub no_prefix: bool,

    // Opt out of using suffix
    pub no_suffix: bool,

    // Set this as the default field if nothing is found
    pub default: Option<Default>,
}

impl VariantAttributes {
    const VARIANTS: &[&str] = &["rename", "alias", "no_prefix", "no_suffix", "default"];

    fn set_rename(&mut self, meta: ParseNestedMeta) -> syn::Result<()> {
        let str: syn::LitStr = meta.value()?.parse()?;
        let value = str.value();
        if value.is_empty() {
            return Err(
                Error::invalid_attribute("rename", "attribute cannot be empty")
                    .to_syn_error(meta.path.span()),
            );
        }

        if self.rename.is_some() {
            return Err(Error::duplicate_attribute(format!("rename::{value}"))
                .to_syn_error(meta.path.span()));
        }

        self.rename = Some(Name {
            value,
            span: Some(meta.path.span()),
        });
        Ok(())
    }

    fn add_alias(&mut self, meta: ParseNestedMeta) -> syn::Result<()> {
        let str: syn::LitStr = meta.value()?.parse()?;
        let value = str.value();
        if value.is_empty() {
            return Err(
                Error::invalid_attribute("alias", "attribute cannot be empty")
                    .to_syn_error(meta.path.span()),
            );
        }

        if self
            .aliases
            .as_ref()
            .is_some_and(|aliases| aliases.iter().any(|a| a.value.eq(&value)))
        {
            return Err(Error::duplicate_attribute(format!("alias::{value}"))
                .to_syn_error(meta.path.span()));
        }

        self.aliases.get_or_insert(Vec::new()).push(Name {
            value,
            span: Some(meta.path.span()),
        });
        Ok(())
    }

    fn disable_prefix(&mut self, meta: ParseNestedMeta) -> syn::Result<()> {
        if self.no_prefix {
            return Err(Error::duplicate_attribute("no_prefix").to_syn_error(meta.path.span()));
        }

        self.no_prefix = true;
        Ok(())
    }

    fn disable_suffix(&mut self, meta: ParseNestedMeta) -> syn::Result<()> {
        if self.no_suffix {
            return Err(Error::duplicate_attribute("no_suffix").to_syn_error(meta.path.span()));
        }

        self.no_suffix = true;
        Ok(())
    }

    fn set_default(&mut self, meta: ParseNestedMeta) -> syn::Result<()> {
        if self.default.is_some() {
            return Err(Error::duplicate_attribute("default").to_syn_error(meta.path.span()));
        }

        self.default = Some(Default {
            span: meta.path.span(),
        });
        Ok(())
    }
}

impl TryFrom<&syn::Variant> for VariantAttributes {
    type Error = syn::Error;

    fn try_from(variant: &syn::Variant) -> Result<Self, Self::Error> {
        let mut va = VariantAttributes::default();
        for attr in &variant.attrs {
            if !attr.path().is_ident("fill") {
                continue;
            }

            attr.parse_nested_meta(|meta| {
                let ident = meta.path.get_ident();
                let ident = quote! { #ident }.to_string();

                match ident.as_ref() {
                    "rename" => va.set_rename(meta),
                    "alias" => va.add_alias(meta),
                    "no_prefix" => va.disable_prefix(meta),
                    "no_suffix" => va.disable_suffix(meta),
                    "default" => va.set_default(meta),
                    _ => {
                        let closest_match = find_closest_match(&ident, Self::VARIANTS);
                        Err(Error::unexpected_attribute(ident, closest_match)
                            .to_syn_error(meta.path.span()))
                    }
                }?;

                Ok(())
            })?;
        }

        Ok(va)
    }
}
