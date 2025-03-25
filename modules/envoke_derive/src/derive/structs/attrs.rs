use syn::{spanned::Spanned, DeriveInput};

use quote::quote;

use crate::{derive::common::Case, errors::Error, utils::find_closest_match};

#[derive(Debug, Default)]
pub struct ContainerAttributes {
    /// Converts environment variable names to the specified case format.
    ///
    /// See [Case] for a full list of supported cases
    ///
    /// ### Example
    ///
    /// The example below will load the environment variable
    /// `PREFIX_FIELD_SUFFIX`
    ///
    /// ```
    /// #[derive(Fill)]
    /// #[fill(prefix = "prefix", suffix = "prefix", delimiter = "_", rename_all = "UPPERCASE")]
    /// struct Example {
    ///     #[fill(env)]
    ///     field: String,
    ///     ...
    /// }
    ///
    /// let _ = Example::try_invoke()?;
    /// ```
    ///
    /// </br>
    ///
    /// **Default:** `None`
    pub rename_all: Option<Case>,

    /// Prefix to prepend to all environment variable names.
    ///
    /// ### Example
    ///
    /// The example below will load the environment variable `TEST_field`
    ///
    /// ```
    /// #[derive(Fill)]
    /// #[fill(prefix = "TEST", delimiter = "_")]
    /// struct Example {
    ///     #[fill(env)]
    ///     field: String,
    ///     ...
    /// }
    ///
    /// let _ = Example::try_invoke()?;
    /// ```
    ///
    /// </br>
    ///
    /// **Default:** `None`
    pub prefix: Option<String>,

    /// Suffix to append to all environment variable names.
    ///
    /// ### Example
    ///
    /// The example below will load the environment variable `field_TEST`
    ///
    /// ```
    /// #[derive(Fill)]
    /// #[fill(suffix = "TEST", delimiter = "_")]
    /// struct Example {
    ///     #[fill(env)]
    ///     field: String,
    ///     ...
    /// }
    ///
    /// let _ = Example::try_invoke()?;
    /// ```
    ///
    /// </br>
    ///
    /// **Default:** `None`
    pub suffix: Option<String>,

    /// Delimiter used to separate the prefix, environment variable name, and
    /// suffix.
    ///
    /// If [`ContainerAttributes::rename_all`] is set, it will override this
    /// delimiter. Although it can still be good to include the delimiter to
    /// separate the prefix/suffix from the original name!
    ///
    /// See [ContainerAttributes::prefix] or [ContainerAttributes::suffix] for
    /// examples on how to use this attribute
    ///
    /// **Default:** `"_"`
    pub delimiter: Option<String>,

    /// Define a dotenv file to load and add to the struct fields
    ///
    /// Note that if an environment variable is found in the processes
    /// environment it will have priority over the variable in the dotenv file
    ///
    /// Expects a standard dotenv file with format  
    /// KEY1=VALUE1  
    /// KEY2=VALUE2  
    ///
    /// **Default**: None
    pub dotenv: Option<String>,
}

impl ContainerAttributes {
    const VARIANTS: &[&str] = &["rename_all", "prefix", "suffix", "delimiter", "dotenv"];

    fn set_rename_all(&mut self, meta: syn::meta::ParseNestedMeta) -> syn::Result<()> {
        if self.rename_all.is_some() {
            return Err(Error::duplicate_attribute("rename_all").to_syn_error(meta.path.span()));
        }

        let case: Case = meta.value()?.parse()?;
        self.rename_all = Some(case);
        Ok(())
    }

    fn set_prefix(&mut self, meta: syn::meta::ParseNestedMeta) -> syn::Result<()> {
        if self.prefix.is_some() {
            return Err(Error::duplicate_attribute("prefix").to_syn_error(meta.path.span()));
        }

        let prefix: syn::LitStr = meta.value()?.parse()?;
        self.prefix = Some(prefix.value());
        Ok(())
    }

    fn set_suffix(&mut self, meta: syn::meta::ParseNestedMeta) -> syn::Result<()> {
        if self.suffix.is_some() {
            return Err(Error::duplicate_attribute("suffix").to_syn_error(meta.path.span()));
        }

        let suffix: syn::LitStr = meta.value()?.parse()?;
        self.suffix = Some(suffix.value());
        Ok(())
    }

    fn set_delimiter(&mut self, meta: syn::meta::ParseNestedMeta) -> syn::Result<()> {
        if self.delimiter.is_some() {
            return Err(Error::duplicate_attribute("delimiter").to_syn_error(meta.path.span()));
        }

        let delimiter: syn::LitStr = meta.value()?.parse()?;
        self.delimiter = Some(delimiter.value());
        Ok(())
    }

    fn set_dotenv(&mut self, meta: syn::meta::ParseNestedMeta) -> syn::Result<()> {
        if self.dotenv.is_some() {
            return Err(Error::duplicate_attribute("dotenv").to_syn_error(meta.path.span()));
        }

        let dotenv: syn::LitStr = meta.value()?.parse()?;
        self.dotenv = Some(dotenv.value());
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
            case.rename(&renamed)
        } else {
            renamed
        }
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
                    "rename_all" => ca.set_rename_all(meta),
                    "prefix" => ca.set_prefix(meta),
                    "suffix" => ca.set_suffix(meta),
                    "delimiter" => ca.set_delimiter(meta),
                    "dotenv" => ca.set_dotenv(meta),
                    _ => {
                        let closest_match = find_closest_match(&ident, Self::VARIANTS);
                        Err(Error::unexpected_attribute(ident, closest_match)
                            .to_syn_error(meta.path.span()))
                    }
                }?;

                Ok(())
            })?;
        }

        Ok(ca)
    }
}

#[derive(Debug)]
pub enum DefaultValue {
    Type(syn::Type),
    Lit(syn::ExprLit),
    Path(syn::ExprPath),
    Call {
        path: syn::ExprPath,
        args: Vec<syn::Expr>,
    },
}

impl syn::parse::Parse for DefaultValue {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let expr: syn::Expr = input.parse()?;
        match expr {
            syn::Expr::Lit(lit) => Ok(DefaultValue::Lit(lit)),
            syn::Expr::Path(path) => Ok(DefaultValue::Path(path)),
            syn::Expr::Call(call) => {
                if let syn::Expr::Path(path) = *call.func {
                    Ok(DefaultValue::Call {
                        path,
                        args: call.args.into_iter().collect(),
                    })
                } else {
                    Err(syn::Error::new_spanned(call, "expected a function"))
                }
            }
            _ => Err(syn::Error::new_spanned(
                expr,
                "unexpected default value format",
            )),
        }
    }
}

#[derive(Debug, Default)]
pub struct ValidateFn {
    /// A function to call after loading the value from the environment variable
    /// to validate it
    pub before: Option<syn::Path>,

    /// A function to call after parsing the value to validate the parsed value
    pub after: Option<syn::Path>,
}

impl ValidateFn {
    const VARIANTS: &[&str] = &["before", "after"];

    fn set_before(&mut self, meta: syn::meta::ParseNestedMeta) -> syn::Result<()> {
        if self.before.is_some() {
            return Err(
                Error::duplicate_attribute("validate_fn::before").to_syn_error(meta.path.span())
            );
        }

        let validate_fn = meta.value()?.parse()?;
        self.before = Some(validate_fn);
        Ok(())
    }

    fn set_after(&mut self, meta: syn::meta::ParseNestedMeta) -> syn::Result<()> {
        if self.after.is_some() {
            return Err(
                Error::duplicate_attribute("validate_fn::after").to_syn_error(meta.path.span())
            );
        }

        let validate_fn = meta.value()?.parse()?;
        self.after = Some(validate_fn);
        Ok(())
    }

    fn from_nested_meta(meta: syn::meta::ParseNestedMeta) -> syn::Result<Self> {
        let mut vfn = Self::default();

        meta.parse_nested_meta(|meta| {
            let ident = meta.path.get_ident();
            let ident = quote! { #ident }.to_string();

            match ident.as_ref() {
                "before" => vfn.set_before(meta),
                "after" => vfn.set_after(meta),
                _ => {
                    let closest_match = find_closest_match(&ident, Self::VARIANTS);
                    Err(Error::unexpected_attribute(ident, closest_match)
                        .to_syn_error(meta.path.span()))
                }
            }?;

            Ok(())
        })?;

        Ok(vfn)
    }

    fn from_direct_assignment(meta: syn::meta::ParseNestedMeta) -> syn::Result<Self> {
        let mut vfn = Self::default();
        vfn.set_after(meta)?;
        Ok(vfn)
    }
}

#[derive(Debug, Default)]
pub struct FieldAttributes {
    /// Environment variables to load the field value from.
    ///
    /// The macro attempts to load each listed environment variable in order.
    /// The first found value is parsed and set as the field value. If parsing
    /// fails, the operation stops, and no further variables are checked.
    ///
    /// **Default:** `None`.
    pub envs: Option<Vec<String>>,

    /// Use the default value if the environment variable is not found
    ///
    /// This function can be used without specifying `envs` to provide a static
    /// fallback.
    ///
    /// **Default:** `None`
    pub default: Option<DefaultValue>,

    /// A function to parse the loaded value with before applying to the field.
    /// Requires `arg_type` to be set if used.
    ///
    /// Allows for custom types which normally isn't supported by envoke-rs
    ///
    /// **Default:** `None`
    pub parse_fn: Option<syn::Path>,

    /// Arg type in the parse_fn function. Required by `parse_fn` if used.
    ///
    /// See [FieldAttributes::parse_fn] for an example on how to use it
    ///
    /// **Default:** `None`
    pub arg_type: Option<syn::Type>,

    /// A function to call after the value is loaded and parsed for extra
    /// validations, e.g., ensuring i64 is above 0
    ///
    /// **Default:** `None`
    pub validate_fn: ValidateFn,

    /// Delimiter used when parsing list-type fields (e.g., `Vec<String>`).
    ///
    /// **Default:** `","`
    pub delimiter: Option<String>,

    /// Disable adding prefix to this environment variables. This will also
    /// remove the delimiter that wouldn't normally be between the environment
    /// variable and prefix
    ///
    /// **Default:** `false`
    pub no_prefix: bool,

    /// Disable adding prefix to this environment variables. This will also
    /// remove the delimiter that wouldn't normally be between the environment
    /// variable and suffix
    ///
    /// **Default:** `false`
    pub no_suffix: bool,

    /// Indicates the the field is a nested struct in which the parser needs to
    /// call try_envoke on
    ///
    /// **Default**: false
    pub is_nested: bool,

    /// Indicates that the field should not be done anything with
    pub is_ignore: bool,
}

impl FieldAttributes {
    const VARIANTS: &[&str] = &[
        "env",
        "default",
        "parse_fn",
        "arg_type",
        "validate_fn",
        "delimiter",
        "no_prefix",
        "no_suffix",
        "nested",
        "ignore",
    ];

    fn add_env(&mut self, field: &syn::Field, meta: syn::meta::ParseNestedMeta) -> syn::Result<()> {
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
                let ident = &field.ident;
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

    fn set_default(
        &mut self,
        field: &syn::Field,
        meta: syn::meta::ParseNestedMeta,
    ) -> syn::Result<()> {
        if self.default.is_some() {
            return Err(Error::duplicate_attribute("default").to_syn_error(meta.path.span()));
        }

        self.default = match meta.input.peek(syn::Token![=]) {
            true => Some(meta.value()?.parse()?),
            false => {
                let ty = &field.ty;
                Some(DefaultValue::Type(ty.clone()))
            }
        };

        Ok(())
    }

    fn set_parse_fn(&mut self, meta: syn::meta::ParseNestedMeta) -> syn::Result<()> {
        if self.parse_fn.is_some() {
            return Err(Error::duplicate_attribute("parse_fn").to_syn_error(meta.path.span()));
        }

        self.parse_fn = Some(meta.value()?.parse()?);
        Ok(())
    }

    fn set_arg_type(&mut self, meta: syn::meta::ParseNestedMeta) -> syn::Result<()> {
        if self.arg_type.is_some() {
            return Err(Error::duplicate_attribute("arg_type").to_syn_error(meta.path.span()));
        }

        self.arg_type = Some(meta.value()?.parse()?);
        Ok(())
    }

    fn set_validate_fn(&mut self, meta: syn::meta::ParseNestedMeta) -> syn::Result<()> {
        if self.validate_fn.before.is_some() || self.validate_fn.after.is_some() {
            return Err(Error::duplicate_attribute("validate_fn").to_syn_error(meta.path.span()));
        }

        self.validate_fn = match meta.input.peek(syn::Token![=]) {
            true => ValidateFn::from_direct_assignment(meta),
            false => ValidateFn::from_nested_meta(meta),
        }?;
        Ok(())
    }

    fn set_delimiter(&mut self, meta: syn::meta::ParseNestedMeta) -> syn::Result<()> {
        if self.delimiter.is_some() {
            return Err(Error::duplicate_attribute("delimiter").to_syn_error(meta.path.span()));
        }

        let str: syn::LitStr = meta.value()?.parse()?;
        let delimiter = str.value();
        if delimiter == "=" {
            return Err(
                Error::invalid_attribute("delimiter", "delimiter reserved by the macro")
                    .to_syn_error(meta.path.span()),
            );
        }

        if delimiter.is_empty() {
            return Err(
                Error::invalid_attribute("delimiter", "attribute cannot be empty")
                    .to_syn_error(meta.path.span()),
            );
        }

        self.delimiter = Some(delimiter);
        Ok(())
    }

    fn disable_prefix(&mut self, meta: syn::meta::ParseNestedMeta) -> syn::Result<()> {
        if self.no_prefix {
            return Err(Error::duplicate_attribute("no_prefix").to_syn_error(meta.path.span()));
        }

        self.no_prefix = true;
        Ok(())
    }

    fn disable_suffix(&mut self, meta: syn::meta::ParseNestedMeta) -> syn::Result<()> {
        if self.no_suffix {
            return Err(Error::duplicate_attribute("no_suffix").to_syn_error(meta.path.span()));
        }

        self.no_suffix = true;
        Ok(())
    }

    fn set_nested(&mut self, meta: syn::meta::ParseNestedMeta) -> syn::Result<()> {
        if self.is_nested {
            return Err(Error::duplicate_attribute("nested").to_syn_error(meta.path.span()));
        }

        self.is_nested = true;
        Ok(())
    }

    fn set_ignore(&mut self, meta: syn::meta::ParseNestedMeta) -> syn::Result<()> {
        if self.is_nested {
            return Err(Error::duplicate_attribute("ignore").to_syn_error(meta.path.span()));
        }

        self.is_ignore = true;
        Ok(())
    }
}

impl TryFrom<&syn::Field> for FieldAttributes {
    type Error = syn::Error;

    fn try_from(field: &syn::Field) -> Result<Self, Self::Error> {
        let mut fa = FieldAttributes::default();
        for attr in &field.attrs {
            if !attr.path().is_ident("fill") {
                continue;
            }

            attr.parse_nested_meta(|meta| {
                let ident = meta.path.get_ident();
                let ident = quote! { #ident }.to_string();

                match ident.as_ref() {
                    "env" => fa.add_env(field, meta),
                    "default" => fa.set_default(field, meta),
                    "parse_fn" => fa.set_parse_fn(meta),
                    "arg_type" => fa.set_arg_type(meta),
                    "validate_fn" => fa.set_validate_fn(meta),
                    "delimiter" => fa.set_delimiter(meta),
                    "no_prefix" => fa.disable_prefix(meta),
                    "no_suffix" => fa.disable_suffix(meta),
                    "nested" => fa.set_nested(meta),
                    "ignore" => fa.set_ignore(meta),
                    _ => {
                        let closest_match = find_closest_match(&ident, Self::VARIANTS);
                        Err(Error::unexpected_attribute(ident, closest_match)
                            .to_syn_error(meta.path.span()))
                    }
                }?;

                Ok(())
            })?;
        }

        // Ensure arg_type is set if parse_fn is used
        match (fa.parse_fn.is_some(), fa.arg_type.is_some()) {
            (true, false) => {
                return Err(
                    Error::missing_attribute("arg_type", "required if `parse_fn` is set")
                        .to_syn_error(field.span()),
                )
            }
            _ => (),
        };

        // If no envs or defaults are given, the field is not marked as nested or to be
        // ignored we add it to the list of envs to load
        if fa.envs.is_none() && fa.default.is_none() && !fa.is_nested && !fa.is_ignore {
            let ident = &field.ident;
            let env = quote! { #ident }.to_string();

            fa.envs.get_or_insert(Vec::new()).push(env);
        }

        Ok(fa)
    }
}
