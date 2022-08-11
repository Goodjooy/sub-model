use darling::FromMeta;
use syn::NestedMeta;

use crate::darling_models::FromIdent;

/// trying to load `T` fro Metalist
///
/// if failure and the first item of Metalist type
/// is `NestedMeta::Lit(..)`
///
/// trying using
/// `FromIdent` construct `T`
pub fn load_from_meta_list<T: FromIdent>(
    meta_list: &impl AsRef<[NestedMeta]>,
) -> darling::Result<T> {
    // try load direct
    <T as FromMeta>::from_list(meta_list.as_ref()).or_else(|err| {
        meta_list
            .as_ref()
            .first()
            // checker whether the error is cause by
            // nested meta lit(only one ident) or not
            .and_then(|meta| {
                if let NestedMeta::Meta(_) = meta {
                    None
                } else {
                    Some(meta)
                }
            })
            // if len of vec nest meta is 0,no try
            .ok_or(err)
            .and_then(|meta| {
                // try load ident only
                <syn::Ident as FromMeta>::from_nested_meta(meta)
                    // mapping to T
                    .map(<T as FromIdent>::form_ident)
            })
    })
}
