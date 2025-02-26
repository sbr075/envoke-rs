use std::str::FromStr;

use convert_case::Case as ConvertCase;
use quote::quote;
use syn::{meta::ParseNestedMeta, parse::Parse};

#[derive(Debug, strum::EnumString)]
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
pub struct ValidateFn {
    /// A function to call after loading the value from the environment variable
    /// to validate it
    pub before: Option<syn::Path>,

    /// A function to call after parsing the value to validate the parsed value
    pub after: Option<syn::Path>,
}

impl ValidateFn {
    fn from_nested_meta(meta: &ParseNestedMeta) -> syn::Result<Self> {
        let mut vfn = Self {
            before: None,
            after: None,
        };

        meta.parse_nested_meta(|meta| {
            if meta.path.is_ident("before") {
                let validate_fn = meta.value()?.parse()?;
                vfn.before = Some(validate_fn);
                return Ok(());
            }

            if meta.path.is_ident("after") {
                let validate_fn = meta.value()?.parse()?;
                vfn.after = Some(validate_fn);
                return Ok(());
            }

            let ident = match meta.path.get_ident() {
                Some(ident) => ident.to_string(),
                None => "unknown".to_string(),
            };
            Err(meta.error(format!("unknown validate_fn attribute `{ident}`")))
        })?;

        Ok(vfn)
    }

    fn from_direct_assignment(meta: &ParseNestedMeta) -> syn::Result<Self> {
        let validate_fn = meta.value()?.parse()?;
        Ok(Self {
            before: None,
            after: Some(validate_fn),
        })
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
                    let parse_fn = meta.value()?.parse()?;
                    fa.parse_fn = Some(parse_fn);
                    return Ok(());
                }

                if meta.path.is_ident("arg_type") {
                    let arg_type = meta.value()?.parse()?;
                    fa.arg_type = Some(arg_type);
                    return Ok(());
                }

                if meta.path.is_ident("validate_fn") {
                    // Check if we can parse validate_fn as a nested meta aka.
                    // #[fill(validate_fn(before = ..., after = ...))]
                    if let Ok(vfn) = ValidateFn::from_nested_meta(&meta) {
                        fa.validate_fn = vfn;
                        return Ok(());
                    }

                    // If first parse fail, try to parse as if its a direct assignment aka.
                    // #[fill(validate_fn = ...)]
                    match ValidateFn::from_direct_assignment(&meta) {
                        Ok(vfn) => {
                            fa.validate_fn = vfn;
                            return Ok(());
                        }
                        Err(_) => {
                            return Err(
                                meta.error("expected either direct assignment or parentheses")
                            )
                        }
                    }
                }

                if meta.path.is_ident("delimiter") {
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
                    fa.no_prefix = true;
                    return Ok(());
                }

                if meta.path.is_ident("no_suffix") {
                    fa.no_suffix = true;
                    return Ok(());
                }

                if meta.path.is_ident("nested") {
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
