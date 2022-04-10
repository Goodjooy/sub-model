use syn::{MetaList, NestedMeta, Path};

use super::darling_unknown_format;

pub fn only_neat_meta_list<T, F>(item: &NestedMeta, f: F) -> darling::Result<T>
where
    F: FnOnce(&Path, &[NestedMeta]) -> darling::Result<T>,
{
    match item {
        syn::NestedMeta::Meta(meta) => match meta {
            syn::Meta::List(MetaList { path, nested, .. }) => {
                let meta_list = nested.into_iter().cloned().collect::<Vec<_>>();
                f(path, &meta_list)
            }
            syn::Meta::Path(_) => darling_unknown_format("path", meta),
            syn::Meta::NameValue(_) => darling_unknown_format("name-value", meta),
        },
        syn::NestedMeta::Lit(_) => darling_unknown_format("lit", item),
    }
}
