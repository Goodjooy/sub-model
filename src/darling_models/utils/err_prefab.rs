use proc_macro2::Ident;

pub fn darling_unknown_format<T, S: syn::spanned::Spanned>(
    ty: &str, span: &S,
) -> Result<T, darling::Error> {
    Err(darling::Error::unsupported_format(ty).with_span(span))
}

pub fn darling_duplicate_field<T>(
    name:&Ident,
)->darling::Result<T>{
    Err(darling::Error::duplicate_field(&name.to_string()))
}

pub fn darling_custom<T>(info:&str)->darling::Result<T>{
    Err(darling::Error::custom(info))
}

