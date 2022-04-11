use std::{collections::HashMap, ops::Deref};

use darling::FromMeta;
use syn::Ident;

use crate::darling_models::utils::{darling_duplicate_field, only_neat_meta_list, ExtraAttrs, Vis};
#[derive(Debug, Clone, FromMeta)]
pub struct NoNameExtraField {
    #[darling(default)]
    pub vis: Vis,
    #[darling(rename = "ty")]
    pub ty: syn::Type,
    #[darling(rename = "from")]
    pub create: syn::Path,
    #[darling(default)]
    pub extra: ExtraAttrs,
}

/// for a better using of extra fields adding
///
/// using the path of nested meta as the extra
///
/// field name
///
/// like this
/// ```rust ignore
/// abb(ty="i32",from="Default::default")
/// ```
/// where the `add` will be the name of extra field,   
///
/// and its type is i32 , construct by func `Default::default`
#[derive(Debug, Clone)]
pub struct ExtraField {
    pub name: Ident,
    pub extra: NoNameExtraField,
}

impl Deref for ExtraField {
    type Target = NoNameExtraField;

    fn deref(&self) -> &Self::Target {
        &self.extra
    }
}

impl FromMeta for ExtraField {
    fn from_nested_meta(item: &syn::NestedMeta) -> darling::Result<Self> {
        only_neat_meta_list(item, |path, items| {
            let name = path
                .get_ident()
                .ok_or(darling::Error::unsupported_format("path").with_span(path))?
                .clone();
            let extra = <NoNameExtraField as FromMeta>::from_list(items)?;

            Ok(Self { name, extra })
        })
    }
}

/// all extra field for a single SubModel
///
/// using `HashMap` for a easy way test
/// duplicate of extra fields
#[derive(Debug, Default)]
pub struct ExtraFields {
    pub inner: HashMap<Ident, ExtraField>,
}

impl FromMeta for ExtraFields {
    fn from_list(items: &[syn::NestedMeta]) -> darling::Result<Self> {
        let mut inner = HashMap::with_capacity(items.len());

        for item in items {
            let item = ExtraField::from_nested_meta(item)?;
            let name = item.name.clone();
            if let Some(_) = inner.insert(name.clone(), item) {
                darling_duplicate_field(&name)?;
            }
        }
        Ok(Self { inner })
    }
}

#[cfg(test)]
mod test_extra {
    use syn::{MetaList, NestedMeta};

    use super::{ExtraField, ExtraFields};
    use darling::FromMeta;

    #[test]
    fn one_extra() {
        let code = r#"mock(vis="pub",ty="i32",from="Default::default")"#;

        let parsed_code = syn::parse_str::<NestedMeta>(code).expect("bad code");
        println!("neta meta {:?}", parsed_code);

        let load = ExtraField::from_nested_meta(&parsed_code).expect("load_error");

        println!("result {:?}", load);
    }

    #[test]
    fn many_extra() {
        let code = r#"
        extra_field(
            cb(ty="i32",from="Default::default",vis="pub(in crate::mock)"),
            b(ty="i32",from="Default::default")
        )
        "#;

        let parsed_code = syn::parse_str::<MetaList>(code)
            .expect("bad code")
            .nested
            .into_iter()
            .collect::<Vec<_>>();

        let mul_load = <ExtraFields as FromMeta>::from_list(&parsed_code)
            .expect("Cannot Load")
            .inner;

        println!("out {:?}", mul_load);
    }
}
