use std::str::FromStr;

use convert_case::Case as ConvertCase;
use quote::quote;
use syn::parse::Parse;

#[derive(Debug, strum::EnumString)]
pub enum Case {
    /// Converts all characters to lowercase and removes binding characters.
    ///
    /// Example: `some_field_name` → `somefieldname`
    #[strum(serialize = "lowercase")]
    Lower,

    /// Converts all characters to uppercase and removes binding characters.
    ///
    /// Example: `some_field_name` → `SOMEFIELDNAME`
    #[strum(serialize = "UPPERCASE")]
    Upper,

    /// Capitalizes the first letter of each word and removes binding
    /// characters.
    ///
    /// Example: `some_field_name` → `SomeFieldName`
    #[strum(serialize = "PascalCase")]
    Pascal,

    /// Lowercases the first letter but capitalizes the first letter of
    /// subsequent words while removing binding characters.
    ///
    /// Example: `some_field_name` → `someFieldName`
    #[strum(serialize = "camelCase")]
    Camel,

    /// Converts names to lowercase and uses underscores `_` to separate words.
    ///
    /// Example: `someFieldName` → `some_field_name`
    #[strum(serialize = "snake_case")]
    Snake,

    /// Converts names to uppercase and uses underscores `_` to separate words.
    ///
    /// Example: `some_field_name` → `SOME_FIELD_NAME`
    #[strum(serialize = "SCREAMING_SNAKE_CASE")]
    ScreamingSnake,

    /// Converts names to lowercase and uses hyphens `-` to separate words.
    ///
    /// Example: `some_field_name` → `some-field-name`
    #[strum(serialize = "kebab-case")]
    Kebab,

    /// Converts names to uppercase and uses hyphens `-` to separate words.
    ///
    /// Example: `some_field_name` → `SOME-FIELD-NAME`
    #[strum(serialize = "SCREAMING-KEBAB-CASE")]
    ScreamingKebab,
}

impl syn::parse::Parse for Case {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let c: syn::LitStr = input.parse()?;
        let case =
            Case::from_str(&c.value()).map_err(|_| input.error("unknown naming convention"))?;

        Ok(case)
    }
}

impl From<&Case> for ConvertCase {
    fn from(value: &Case) -> Self {
        match value {
            Case::Lower => ConvertCase::Flat,
            Case::Upper => ConvertCase::UpperFlat,
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
    /// **Default:** `None`
    pub prefix: Option<String>,

    /// Suffix to append to all environment variable names.
    ///
    /// **Default:** `None`
    pub suffix: Option<String>,

    /// Delimiter used to separate the prefix, environment variable name, and
    /// suffix.
    ///
    /// If [`ContainerAttributes::case`] is set, it will override this
    /// delimiter.
    ///
    /// **Default:** `"_"`
    pub delimiter: Option<String>,

    /// Converts environment variable names to the specified case format.
    ///
    /// **Default:** `None`
    pub rename_all: Option<Case>,
}

impl ContainerAttributes {
    pub fn get_prefix(&self) -> &str {
        self.prefix.as_deref().unwrap_or_default()
    }

    pub fn get_suffix(&self) -> &str {
        self.suffix.as_deref().unwrap_or_default()
    }

    pub fn get_delimiter(&self) -> &str {
        self.delimiter.as_deref().unwrap_or_default()
    }

    pub fn parse_attrs(attrs: &Vec<syn::Attribute>) -> syn::Result<Self> {
        let mut ca = ContainerAttributes::default();

        for attr in attrs {
            if !attr.path().is_ident("fill") {
                continue;
            }

            attr.parse_nested_meta(|meta| {
                if meta.path.is_ident("prefix") {
                    if ca.prefix.is_some() {
                        return Err(meta.error("container attribute `prefix` already set"));
                    }

                    let str: syn::LitStr = meta.value()?.parse()?;
                    ca.prefix = Some(str.value());
                    return Ok(());
                }

                if meta.path.is_ident("suffix") {
                    if ca.suffix.is_some() {
                        return Err(meta.error("container attribute `suffix` already set"));
                    }

                    let str: syn::LitStr = meta.value()?.parse()?;
                    ca.suffix = Some(str.value());
                    return Ok(());
                }

                if meta.path.is_ident("delimiter") {
                    if ca.delimiter.is_some() {
                        return Err(meta.error("container attribute `delimiter` already set"));
                    }

                    let str: syn::LitStr = meta.value()?.parse()?;
                    ca.delimiter = Some(str.value());
                    return Ok(());
                }

                if meta.path.is_ident("rename_all") {
                    if ca.rename_all.is_some() {
                        return Err(meta.error("container attribute `rename_all` already set"));
                    }

                    let case: Case = meta.value()?.parse()?;
                    ca.rename_all = Some(case);
                    return Ok(());
                }

                let ident = match meta.path.get_ident() {
                    Some(ident) => ident.to_string(),
                    None => "unknown".to_string(),
                };
                Err(meta.error(format!("unknown container attribute `{ident}`")))
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

impl Parse for DefaultValue {
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
                    Err(syn::Error::new_spanned(call, "Expected a function path"))
                }
            }
            _ => Err(syn::Error::new_spanned(
                expr,
                "Unexpected default value format",
            )),
        }
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
    /// **Default:** The field name.
    pub envs: Option<Vec<String>>,

    /// Use the default value if the environment variable is not found
    ///
    /// This function can be used without specifying `envs` to provide a static
    /// fallback.
    ///
    /// **Default:** `None`
    pub default: Option<DefaultValue>,

    /// A function to parse the loaded value with before applying to the field
    ///
    /// Allows for custom types which normally isn't supported by envoke-rs
    ///
    /// **Default:** `None`
    pub parse_fn: Option<syn::Path>,

    /// Arg type in the parse_fn function
    ///
    /// **Default:** `None`
    pub arg_type: Option<syn::Type>,

    /// Delimiter used when parsing list-type fields (e.g., `Vec<String>`).
    ///
    /// **Default:** `","`
    pub delimiter: Option<String>,

    /// Disable adding prefix to this environemnt variables. This will also
    /// remove the delimiter that wouldn't normally be between the environment
    /// variable and prefix
    ///
    /// **Default:** `false`
    pub no_prefix: bool,

    /// Disable adding prefix to this environemnt variables. This will also
    /// remove the delimiter that wouldn't normally be between the environment
    /// variable and suffix
    ///
    /// **Default:** `false`
    pub no_suffix: bool,

    /// Indicates the the field is a nested struct in which the parser needs to
    /// call try_envoke on
    ///
    /// **Default**: false
    pub nested: bool,
}

impl FieldAttributes {
    fn add_env(&mut self, env: String) {
        self.envs.get_or_insert_with(Vec::new).push(env);
    }

    pub fn parse_attrs(field: &syn::Field, attrs: &Vec<syn::Attribute>) -> syn::Result<Self> {
        let mut fa = FieldAttributes::default();
        for attr in attrs {
            if !attr.path().is_ident("fill") {
                continue;
            }

            attr.parse_nested_meta(|meta| {
                if meta.path.is_ident("env") {
                    // Allows the user to specify both
                    // 1. `#[fill(env)]` - Uses the field name as environment variable
                    // 2. `#[fill(env = "env")]` - Uses `env` as the environment variable
                    match meta.input.peek(syn::Token![=]) {
                        true => {
                            let str: syn::LitStr = meta.value()?.parse()?;
                            let env = str.value();
                            if env.is_empty() {
                                return Err(meta.error("field attribute `env` cannot be empty"));
                            }

                            fa.add_env(env);
                        }
                        false => {
                            let ident = &field.ident;
                            let name = quote! { #ident }.to_string();
                            fa.add_env(name.to_owned())
                        }
                    }

                    return Ok(());
                }

                // Allows the user to specify both
                // 1. `#[fill(default)]` - Uses the field types default value
                // 2. `#[fill(default = default_t)]` - Uses `default_t` as the field value
                // 3. `#[fill(default = default_fn)]` - Uses `default_fn` return value as the
                //    field value
                if meta.path.is_ident("default") {
                    if fa.default.is_some() {
                        return Err(meta.error("field attribute `default` already set"));
                    }

                    fa.default = match meta.input.peek(syn::Token![=]) {
                        true => Some(meta.value()?.parse()?),
                        false => {
                            let ty = &field.ty;
                            Some(DefaultValue::Type(ty.clone()))
                        }
                    };

                    return Ok(());
                }

                if meta.path.is_ident("parse_fn") {
                    if fa.parse_fn.is_some() {
                        return Err(meta.error("field attribute `parse_fn` already set"));
                    }

                    let parse_fn = meta.value()?.parse()?;
                    fa.parse_fn = Some(parse_fn);
                    return Ok(());
                }

                if meta.path.is_ident("arg_type") {
                    if fa.arg_type.is_some() {
                        return Err(meta.error("field attribute `arg_type` already set"));
                    }

                    let arg_type = meta.value()?.parse()?;
                    fa.arg_type = Some(arg_type);
                    return Ok(());
                }

                if meta.path.is_ident("delimiter") {
                    if fa.delimiter.is_some() {
                        return Err(meta.error("field attribute `delimiter` already set"));
                    }

                    let str: syn::LitStr = meta.value()?.parse()?;
                    let delimiter = str.value();
                    if delimiter == "=" {
                        return Err(
                            meta.error("delimiter `=` is reserved by the macro and cannot be used")
                        );
                    }

                    if delimiter.is_empty() {
                        return Err(meta.error("field attribute `delimiter` cannot be empty"));
                    }

                    fa.delimiter = Some(delimiter);
                    return Ok(());
                }

                if meta.path.is_ident("no_prefix") {
                    if fa.no_prefix {
                        return Err(meta.error("field attribute `no_prefix` already set"));
                    }

                    fa.no_prefix = true;
                    return Ok(());
                }

                if meta.path.is_ident("no_suffix") {
                    if fa.no_suffix {
                        return Err(meta.error("field attribute `no_suffix` already set"));
                    }

                    fa.no_suffix = true;
                    return Ok(());
                }

                if meta.path.is_ident("nested") {
                    if fa.nested {
                        return Err(meta.error("field attribute `nested` already set"));
                    }

                    fa.nested = true;
                    return Ok(());
                }

                let ident = match meta.path.get_ident() {
                    Some(ident) => ident.to_string(),
                    None => "unknown".to_string(),
                };
                Err(meta.error(format!("unknown field attribute `{ident}`")))
            })?;
        }

        Ok(fa)
    }
}
