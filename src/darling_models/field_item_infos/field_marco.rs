use darling::FromMeta;
use syn::MetaList;

use super::{having_field::HaveField, ignore_field::IgnoreField};
use crate::darling_models::utils::{darling_unknown_format, load_from_meta_list};

#[derive(Debug)]
/// the type of have a field
pub enum HaveStatus {
    /// the SubModel Having It.
    /// only use for extra info for *all*
    Having,
    /// the SubModel Want It.
    /// only use for  *none*
    Want,
}

#[derive(Debug)]
/// a multi type for SubModel Info
pub enum FieldType {
    /// the SubModel want Ignore It.
    /// only use for *all*
    Ignore(IgnoreField),
    /// the SubModel have It.  
    /// dependent on `HaveStatus`
    Have(HaveStatus, HaveField),
}

impl FieldType {
    fn new_want(want: HaveField) -> darling::Result<Self> {
        Ok(Self::Have(HaveStatus::Want, want))
    }

    fn new_having(having: HaveField) -> darling::Result<Self> {
        Ok(Self::Have(HaveStatus::Having, having))
    }

    fn new_ignore(ignore: IgnoreField) -> darling::Result<Self> {
        Ok(Self::Ignore(ignore))
    }

    pub fn get_owner(&self) -> syn::Ident {
        match self {
            FieldType::Ignore(ig) => &ig.owner,
            FieldType::Have(_, hv) => &hv.owner,
        }
        .to_owned()
    }
}

impl FromMeta for FieldType {
    fn from_nested_meta(item: &syn::NestedMeta) -> darling::Result<Self> {
        match item {
            syn::NestedMeta::Meta(meta) => {
                match meta {
                    syn::Meta::List(MetaList { path, nested, .. }) => {
                        let meta_list = nested.into_iter().cloned().collect::<Vec<_>>();
                        // loading want(for="...",...)
                        if path.is_ident("want") {
                            Self::new_want(
                                load_from_meta_list(&meta_list).map_err(|e| e.with_span(meta))?,
                            )
                        }
                        // loading having("Mock")
                        else if path.is_ident("having") {
                            Self::new_having(
                                load_from_meta_list(&meta_list).map_err(|e| e.with_span(meta))?,
                            )
                        }
                        // loading ignore("Mock")
                        else if path.is_ident("ignore") {
                            Self::new_ignore(
                                load_from_meta_list(&meta_list).map_err(|e| e.with_span(meta))?,
                            )
                        }
                        // unknown type name, with error
                        else {
                            Err(darling::Error::unknown_field(
                                &path.get_ident().unwrap().to_string(),
                            )
                            .with_span(meta))
                        }
                    }
                    syn::Meta::Path(_) => darling_unknown_format("path", meta),
                    syn::Meta::NameValue(_) => darling_unknown_format("nameValue", meta),
                }
            }
            syn::NestedMeta::Lit(_) => darling_unknown_format("lit", item),
        }
    }
}

pub struct FieldMarcos {
    pub inner: Vec<FieldType>,
}

impl FromMeta for FieldMarcos {
    fn from_list(items: &[syn::NestedMeta]) -> darling::Result<Self> {
        let mut inner = Vec::with_capacity(items.len());
        for item in items {
            inner.push(FromMeta::from_nested_meta(item).map_err(|e| e.with_span(item))?)
        }
        Ok(Self { inner })
    }
}


#[cfg(test)]
mod test{
    
}
