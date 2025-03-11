use std::str::FromStr;

use convert_case::{Case as ConvertCase, Casing};
use strum::VariantNames;

use crate::utils::find_closest_match;

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

impl Case {
    pub fn rename(&self, s: &str) -> String {
        match self {
            Case::Lower => s.to_lowercase(),
            Case::Upper => s.to_uppercase(),
            Case::Pascal => s.to_case(ConvertCase::Pascal),
            Case::Camel => s.to_case(ConvertCase::Camel),
            Case::Snake => s.to_case(ConvertCase::Snake),
            Case::ScreamingSnake => s.to_case(ConvertCase::UpperSnake),
            Case::Kebab => s.to_case(ConvertCase::Kebab),
            Case::ScreamingKebab => s.to_case(ConvertCase::UpperKebab),
        }
    }
}
