use thiserror::Error;

#[derive(Debug, Error)]
pub enum AttributeError {
    #[error("duplicate attribute `{attr}`")]
    Duplicate { attr: String },

    #[error(
        "unexpected attribute `{attr}`{}", 
        closest_match
            .as_ref()
            .map_or("".to_string(), |m| format!(", did you mean `{m}`?"))
        )
    ]
    Unexpected {
        attr: String,
        closest_match: Option<String>,
    },

    #[error("attribute `{attr}` is already used before")]
    AlreadyUsed { attr: String },

    #[error("invalid attribute `{attr}`: {reason}")]
    Invalid { attr: String, reason: String },

    #[error("missing attribute `{attr}`: {reason}")]
    Missing { attr: String, reason: String },
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("Error: unsupported target, fill can only be derived for structs and enums")]
    UnsupportedTarget,

    #[error("Error: unsupported struct type, fill can only be derived for named structs")]
    UnsupportedStructType,

    #[error("Error: unsupported enum type, fill can only be derived for unnamed enums")]
    UnsupportedEnumType,

    #[error("Error: {0}")]
    Attribute(#[from] AttributeError),

    #[error(
        "Error: field is missing key attribute(s): atleast one of the field attributes `env`, \
         `default`, or `nested` is required"
    )]
    IncompleteField,
}

impl Error {
    pub fn duplicate_attribute(attr: impl ToString) -> Self {
        Error::Attribute(AttributeError::Duplicate {
            attr: attr.to_string(),
        })
    }

    pub fn unexpected_attribute(attr: impl ToString, closest_match: Option<impl ToString>) -> Self {
        Error::Attribute(AttributeError::Unexpected {
            attr: attr.to_string(),
            closest_match: closest_match.map(|m| m.to_string()),
        })
    }

    pub fn already_used(attr: impl ToString) -> Self {
        Error::Attribute(AttributeError::AlreadyUsed {
            attr: attr.to_string(),
        })
    }

    pub fn invalid_attribute(attr: impl ToString, reason: impl ToString) -> Self {
        Error::Attribute(AttributeError::Invalid {
            attr: attr.to_string(),
            reason: reason.to_string(),
        })
    }

    pub fn missing_attribute(attr: impl ToString, reason: impl ToString) -> Self {
        Error::Attribute(AttributeError::Missing {
            attr: attr.to_string(),
            reason: reason.to_string(),
        })
    }

    pub fn to_syn_error(&self, span: proc_macro2::Span) -> syn::Error {
        syn::Error::new(span, self)
    }
}
