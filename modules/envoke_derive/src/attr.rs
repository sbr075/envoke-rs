use std::str::FromStr;

use convert_case::Case as ConvertCase;
use quote::quote;
use strum::VariantNames;
use syn::spanned::Spanned;

use crate::errors::Error;

#[derive(Debug, strum::EnumString, strum::VariantNames)]
pub enum Case {
    /// Converts all characters to lowercase and removes binding characters.
    ///
    /// Used if [ContainerAttributes::rename_all] is set to `lowercase` or
    /// `lower`
    ///
    /// ### Example
    ///
    /// Renames `EXAMPLE_ENV` to `exampleenv`
    ///
    /// ```
    /// #[derive(Fill)]
    /// #[fill(rename_all = "lowercase")]
    /// struct Example {
    ///     #[fill(env = "EXAMPLE_ENV")]
    ///     field: String,
    /// }
    ///
    /// let _ = Example::try_invoke()?;
    /// ```
    #[strum(serialize = "lowercase", serialize = "lower")]
    Lower,

    /// Converts all characters to uppercase and removes binding characters.
    ///
    /// Used if [ContainerAttributes::rename_all] is set to `UPPERCASE` or
    /// `UPPER`
    ///
    /// ### Example
    ///
    /// Renames `example_env` to `EXAMPLEENV`
    ///
    /// ```
    /// #[derive(Fill)]
    /// #[fill(rename_all = "UPPERCASE")]
    /// struct Example {
    ///     #[fill(env = "example_env")]
    ///     field: String,
    /// }
    ///
    /// let _ = Example::try_invoke()?;
    /// ```
    #[strum(serialize = "UPPERCASE", serialize = "UPPER")]
    Upper,

    /// Capitalizes the first letter of each word and removes binding
    /// characters.
    ///
    /// Used if [ContainerAttributes::rename_all] is set to `PascalCase`
    ///
    /// ### Example
    ///
    /// Renames `some_field_name` to `SomeFieldName`
    ///
    /// ```
    /// #[derive(Fill)]
    /// #[fill(rename_all = "PascalCase")]
    /// struct Example {
    ///     #[fill(env = "some_field_name")]
    ///     field: String,
    /// }
    ///
    /// let _ = Example::try_invoke()?;
    /// ```
    #[strum(serialize = "PascalCase")]
    Pascal,

    /// Lowercases the first letter but capitalizes the first letter of
    /// subsequent words while removing binding characters.
    ///
    /// Used if [ContainerAttributes::rename_all] is set to `camelCase`
    ///
    /// ### Example
    ///
    /// Renames `some_field_name` to `someFieldName`
    ///
    /// ```
    /// #[derive(Fill)]
    /// #[fill(rename_all = "camelCase")]
    /// struct Example {
    ///     #[fill(env = "some_field_name")]
    ///     field: String,
    /// }
    ///
    /// let _ = Example::try_invoke()?;
    /// ```
    #[strum(serialize = "camelCase")]
    Camel,

    /// Converts names to lowercase and uses underscores `_` to separate words.
    ///
    /// Used if [ContainerAttributes::rename_all] is set to `snake_case`
    ///
    /// ### Example
    ///
    /// Renames `someFieldName` to `some_field_name`
    ///
    /// ```
    /// #[derive(Fill)]
    /// #[fill(rename_all = "snake_case")]
    /// struct Example {
    ///     #[fill(env = "someFieldName")]
    ///     field: String,
    /// }
    ///
    /// let _ = Example::try_invoke()?;
    /// ```
    #[strum(serialize = "snake_case")]
    Snake,

    /// Converts names to uppercase and uses underscores `_` to separate words.
    ///
    /// Used if [ContainerAttributes::rename_all] is set to
    /// `SCREAMING_SNAKE_CASE`
    ///
    /// ### Example
    ///
    /// Renames `some_field_name` to `SOME_FIELD_NAME`
    ///
    /// ```
    /// #[derive(Fill)]
    /// #[fill(rename_all = "SCREAMING_SNAKE_CASE")]
    /// struct Example {
    ///     #[fill(env = "some_field_name")]
    ///     field: String,
    /// }
    ///
    /// let _ = Example::try_invoke()?;
    /// ```
    #[strum(serialize = "SCREAMING_SNAKE_CASE")]
    ScreamingSnake,

    /// Converts names to lowercase and uses hyphens `-` to separate words.
    ///
    /// Used if [ContainerAttributes::rename_all] is set to `kebab-case`
    ///
    /// ### Example
    ///
    /// Renames `some_field_name` to `some-field-name`
    ///
    /// ```
    /// #[derive(Fill)]
    /// #[fill(rename_all = "kebab-case")]
    /// struct Example {
    ///     #[fill(env = "some_field_name")]
    ///     field: String,
    /// }
    ///
    /// let _ = Example::try_invoke()?;
    /// ```
    #[strum(serialize = "kebab-case")]
    Kebab,

    /// Converts names to uppercase and uses hyphens `-` to separate words.
    ///
    /// Used if [ContainerAttributes::rename_all] is set to
    /// `SCREAMING-KEBAB-CASE`
    ///
    /// ### Example
    ///
    /// Renames `some_field_name` to `SOME-FIELD-NAME`
    ///
    /// ```
    /// #[derive(Fill)]
    /// #[fill(rename_all = "SCREAMING-KEBAB-CASE")]
    /// struct Example {
    ///     #[fill(env = "some_field_name")]
    ///     field: String,
    /// }
    ///
    /// let _ = Example::try_invoke()?;
    /// ```
    #[strum(serialize = "SCREAMING-KEBAB-CASE")]
    ScreamingKebab,
}

fn find_closest_match(input: &str, variants: &'static [&'static str]) -> Option<&'static str> {
    for variant in variants {
        let distance = strsim::levenshtein(input, &variant);
        if distance <= 5 {
            return Some(variant);
        }
    }

    None
}

impl syn::parse::Parse for Case {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let input: syn::LitStr = input.parse()?;
        let value = input.value();
        Case::from_str(&value).map_err(|_| {
            let mut message = format!("unexpected naming convention `{value}`");
            if let Some(closest_match) = find_closest_match(&value, Case::VARIANTS) {
                message = format!("{message}, did you mean `{closest_match}`?")
            }

            syn::Error::new_spanned(input, message)
        })
    }
}

impl From<&Case> for ConvertCase {
    fn from(value: &Case) -> Self {
        match value {
            Case::Lower => ConvertCase::Lower,
            Case::Upper => ConvertCase::Upper,
            Case::Pascal => ConvertCase::Pascal,
            Case::Camel => ConvertCase::Camel,
            Case::Snake => ConvertCase::Snake,
            Case::ScreamingSnake => ConvertCase::UpperSnake,
            Case::Kebab => ConvertCase::Kebab,
            Case::ScreamingKebab => ConvertCase::UpperKebab,
        }
    }
}

#[derive(Debug, Default)]
pub struct ContainerAttributes {
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
}

impl ContainerAttributes {
    const VARIANTS: &[&str] = &["prefix", "suffix", "delimiter", "rename_all"];

    pub fn get_prefix(&self) -> &str {
        self.prefix.as_deref().unwrap_or_default()
    }

    fn set_prefix(&mut self, meta: syn::meta::ParseNestedMeta) -> syn::Result<()> {
        if self.prefix.is_some() {
            return Err(Error::duplicate_attribute("prefix").to_syn_error(meta.path.span().span()));
        }

        let prefix: syn::LitStr = meta.value()?.parse()?;
        self.prefix = Some(prefix.value());
        Ok(())
    }

    pub fn get_suffix(&self) -> &str {
        self.suffix.as_deref().unwrap_or_default()
    }

    fn set_suffix(&mut self, meta: syn::meta::ParseNestedMeta) -> syn::Result<()> {
        if self.suffix.is_some() {
            return Err(Error::duplicate_attribute("suffix").to_syn_error(meta.path.span().span()));
        }

        let suffix: syn::LitStr = meta.value()?.parse()?;
        self.suffix = Some(suffix.value());
        Ok(())
    }

    pub fn get_delimiter(&self) -> &str {
        self.delimiter.as_deref().unwrap_or_default()
    }

    fn set_delimiter(&mut self, meta: syn::meta::ParseNestedMeta) -> syn::Result<()> {
        if self.delimiter.is_some() {
            return Err(
                Error::duplicate_attribute("delimiter").to_syn_error(meta.path.span().span())
            );
        }

        let delimiter: syn::LitStr = meta.value()?.parse()?;
        self.delimiter = Some(delimiter.value());
        Ok(())
    }

    fn set_rename_all(&mut self, meta: syn::meta::ParseNestedMeta) -> syn::Result<()> {
        if self.rename_all.is_some() {
            return Err(Error::duplicate_attribute("rename_all").to_syn_error(meta.path.span()));
        }

        let case: Case = meta.value()?.parse()?;
        self.rename_all = Some(case);
        Ok(())
    }

    pub fn parse_attrs(attrs: &Vec<syn::Attribute>) -> syn::Result<Self> {
        let mut ca = ContainerAttributes::default();

        for attr in attrs {
            if !attr.path().is_ident("fill") {
                continue;
            }

            attr.parse_nested_meta(|meta| {
                let ident = meta.path.get_ident();
                let ident = quote! { #ident }.to_string();

                match ident.as_ref() {
                    "prefix" => ca.set_prefix(meta),
                    "suffix" => ca.set_suffix(meta),
                    "delimiter" => ca.set_delimiter(meta),
                    "rename_all" => ca.set_rename_all(meta),
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
    /// ### Usage
    ///
    /// **1.** `env`
    ///
    /// The example below will load the value from environment variable `field`
    /// ```
    /// #[derive(Fill)]
    /// struct Example {
    ///     #[fill(env)]
    ///     field: i32,
    ///     ...
    /// }
    ///
    /// let _ = Example::try_invoke()?;
    /// ```
    ///
    /// </br>
    ///
    /// **2.** `env = "<key>"`
    ///
    /// The example below will load the value from environment variable `ENV`
    /// ```
    /// #[derive(Fill)]
    /// struct Example {
    ///     #[fill(env = "ENV")]
    ///     field: i32,
    ///     ...
    /// }
    ///
    /// let _ = Example::try_invoke()?;
    /// ```
    ///
    /// `env` and `env = "<key>"` can be used together, as well as can be added
    /// on as many times as needed. Note that they will be loaded in the order
    /// they are defined
    ///
    /// </br>
    ///
    /// **Default:** `None`.
    pub envs: Option<Vec<String>>,

    /// Use the default value if the environment variable is not found
    ///
    /// This function can be used without specifying `envs` to provide a static
    /// fallback.
    ///
    /// ### Usage
    ///
    /// **1.** `default`
    ///
    /// The example below will `field` with `i32::default()` which is `0`
    /// ```
    /// #[derive(Fill)]
    /// struct Example {
    ///     #[fill(default)]
    ///     field: i32,
    ///     ...
    /// }
    ///
    /// let _ = Example::try_invoke()?;
    /// ```
    ///
    /// </br>
    ///
    /// **2.** `default = <value>`
    ///
    /// The example below will `field` with `10`
    /// ```
    /// #[derive(Fill)]
    /// struct Example {
    ///     #[fill(default = 10)]
    ///     field: i32,
    ///     ...
    /// }
    ///
    /// let _ = Example::try_invoke()?;
    /// ```
    ///
    /// </br>
    ///
    /// **3.** `default = <func>()`
    ///
    /// The example below will `field` with `10` (if result remains unchanged)
    /// ```
    /// fn field_default() -> i32 {
    ///     let result = 10;
    ///     ... // extra steps if need
    ///     result
    /// }
    ///
    /// #[derive(Fill)]
    /// struct Example {
    ///     #[fill(default = field_default())]
    ///     field: i32
    ///     ...
    /// }
    ///
    /// let _ = Example::try_invoke()?;
    /// ```
    ///
    /// Additionally arguments can be parsed to field_default() if needed
    ///
    /// </br>
    ///
    /// **Default:** `None`
    pub default: Option<DefaultValue>,

    /// A function to parse the loaded value with before applying to the field.
    /// Requires `arg_type` to be set if used.
    ///
    /// Allows for custom types which normally isn't supported by envoke-rs
    ///
    /// ### Example
    ///
    /// The example below takes the value from environment variable `field` and
    /// runs it through the `to_duration` function before assigning it to the
    /// field value
    ///
    /// ```
    /// fn to_duration(secs: u64) -> std::time::Duration {
    ///     std::time::Duration::from_secs(secs)
    /// }
    ///
    /// #[derive(Fill)]
    /// struct Example {
    ///     #[fill(env, parse_fn = to_duration, arg_type = u64)]
    ///     field: std::time::Duration,
    ///     ...
    /// }
    ///
    /// let _ = Example::try_invoke()?;
    /// ```
    ///
    /// </br>
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
    /// ### Example
    ///
    /// The example below takes and i64 and checks that it is above 0
    ///
    /// ```
    /// fn above_zero(secs: u64) -> std::result::Result<()> {
    ///     match secs > 0 {
    ///         true => Ok(),
    ///         false => Err("duration cannot be less than 0")
    ///     }
    /// }
    ///
    /// #[derive(Fill)]
    /// struct Example {
    ///     #[fill(env, parse_fn = to_duration, arg_type = u64, validate_fn = above_zero)]
    ///     field: std::time::Duration,
    ///     ...
    /// }
    /// ```
    ///
    /// **Default:** `None`
    pub validate_fn: ValidateFn,

    /// Delimiter used when parsing list-type fields (e.g., `Vec<String>`).
    ///
    /// ### Example
    ///
    /// The example below will parse, e.g., `value1;value2;value3` to
    /// Vec<String>. Note that the delimiter `=` is reserved as its the
    /// delimiter for key and values in a map string
    ///
    /// ```
    /// #[derive(Fill)]
    /// struct Example {
    ///     #[fill(env, delimiter = ";")]
    ///     field: Vec<String>,
    ///     ...
    /// }
    /// ```
    ///
    /// </br>
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
    /// ### Example
    ///
    /// ```rust
    /// #[derive(Fill)]
    /// struct Inner {
    ///     #[fill(env)]
    ///     field: String,
    ///     ...
    /// }
    ///
    /// #[derive(Fill)]
    /// struct Outer {
    ///     #[fill(nested)]
    ///     inner: Inner,
    ///     ...
    /// }
    /// ```
    ///
    /// The structs can nested multiple times
    ///
    /// </br>
    ///
    /// **Default**: false
    pub is_nested: bool,
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

    fn is_nested(&mut self, meta: syn::meta::ParseNestedMeta) -> syn::Result<()> {
        if self.is_nested {
            return Err(Error::duplicate_attribute("nested").to_syn_error(meta.path.span()));
        }

        self.is_nested = true;
        Ok(())
    }

    pub fn parse_attrs(field: &syn::Field, attrs: &Vec<syn::Attribute>) -> syn::Result<Self> {
        let mut fa = FieldAttributes::default();
        for attr in attrs {
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
                    "nested" => fa.is_nested(meta),
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

        if fa.envs.is_none() && fa.default.is_none() && !fa.is_nested {
            return Err(Error::IncompleteField.to_syn_error(field.span()));
        }

        Ok(fa)
    }
}
