use std::collections::HashMap;

use darling::FromMeta;
use syn::{Ident, MetaList};

use crate::darling_models::utils::{
    darling_duplicate_field, darling_unknown_format, load_from_meta_list,
};

use super::{IgnoreField, HaveField};


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
pub enum FieldInput {
    /// the SubModel want Ignore It.
    /// only use for *all*
    Ignore(IgnoreField),
    /// the SubModel have It.  
    /// dependent on `HaveStatus`
    Have(HaveStatus, HaveField),
}

impl FieldInput {
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
            FieldInput::Ignore(ig) => &ig.owner,
            FieldInput::Have(_, hv) => &hv.owner,
        }
        .to_owned()
    }
}

impl FromMeta for FieldInput {
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

/// all sub model on specify field
pub struct FieldInputs {
    pub inner: HashMap<Ident, FieldInput>,
}

impl FromMeta for FieldInputs {
    fn from_list(items: &[syn::NestedMeta]) -> darling::Result<Self> {
        let mut inner = HashMap::with_capacity(items.len());
        for item in items {
            let ft = FieldInput::from_nested_meta(item).map_err(|e| e.with_span(item))?;
            if let Some(ft) = inner.insert(ft.get_owner(), ft) {
                darling_duplicate_field(&ft.get_owner())?;
            }
        }
        Ok(Self { inner })
    }
}

#[cfg(test)]
mod test {
    use darling::FromMeta;
    use syn::{Ident, MetaList, NestedMeta};

    use super::{FieldInputs, FieldInput};

    #[test]
    fn test_one_simple() {
        let code = code!(NestedMeta:r#"
        want("Mock")
        "#
        );
        let mock = code!(Ident:"Mock");

        let ft = FieldInput::from_nested_meta(&code).unwrap();

        assert_eq!(ft.get_owner(), mock);

        println!("out :{:?}", &ft);
    }

    #[test]
    fn test_one_complex() {
        let item = code!(NestedMeta:r#"
        having(
            for="Mock",
            vis="pub",
            extra(
                doc="aac"
            ),
            rename = "cca",
            to_type(
                ty="i32",
                by="Into::into"
            )
        )
        "#);
        let mock = code!(Ident:"Mock");
        let ft = FieldInput::from_nested_meta(&item).unwrap();

        assert_eq!(ft.get_owner(), mock);

        println!("out {:?}", &ft);
    }

    #[test]
    fn test_mix_complex() {
        let item = code!(
            MetaList:
            r#"
            sub_model(
                want("Acc"),
                ignore(for="Ea"),
                having(
                    for="Me",
                    vis="pub",
                    rename="ccd",
                    to_type(
                        ty="u32",
                        by="Into::into"
                    )
                )
            )
            "#
        )
        .nested
        .into_iter()
        .collect::<Vec<_>>();

        let fts = FieldInputs::from_list(&item).unwrap().inner;

        assert_eq!(fts.len(), 3);

        for (k,v) in fts {
            println!("owner : {:?}\n\n {:?}\n",k,v);
        }
    }
}
