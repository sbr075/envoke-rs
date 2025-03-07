use syn::Type;

pub fn find_closest_match(input: &str, variants: &'static [&'static str]) -> Option<&'static str> {
    for variant in variants {
        let distance = strsim::levenshtein(input, &variant);
        if distance <= 5 {
            return Some(variant);
        }
    }

    None
}

pub fn is_optional(ty: &Type) -> bool {
    match ty {
        Type::Path(path) => path.path.segments[0].ident == "Option",
        _ => false,
    }
}
