use quote::spanned::Spanned;
use syn::{Path, PathArguments, PathSegment};

use super::err_prefab::darling_unexpected_type;

pub fn path_to_ident(path: &Path) -> darling::Result<syn::Ident> {
    let Path {
        leading_colon,
        segments,
    } = path;
    let mut iter = segments.iter();
    if let (
        None,
        Some(PathSegment {
            ident,
            arguments: PathArguments::None,
        }),
        None,
    ) = (leading_colon, iter.next(), iter.next())
    {
        Ok(ident.clone())
    } else {
        darling_unexpected_type("syn::Path", &path.__span())
    }
}
