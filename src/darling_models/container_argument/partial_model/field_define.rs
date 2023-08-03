use std::ops::Deref;
use darling::FromMeta;
use proc_macro2::Ident;
use quote::IdentFragment;
use syn::{Meta, MetaList, NestedMeta, Type};
use syn::spanned::Spanned;
use tap::{Tap, TapFallible};

use constructor::Constructor;
use requires::RequireList;

use crate::darling_models::utils::{darling_unknown_format, from_nest_meta_list};

mod requires;
mod constructor;

#[derive(Debug, FromMeta)]
pub struct FieldDefine {
    pub(crate) ty: Type,
    pub(crate) constructor: Constructor,
    #[darling(default)]
    pub(crate) requires: Option<RequireList>,
    #[darling(default)]
    pub(crate) require_self: bool,
}

#[derive(Debug)]
pub struct FieldItem {
    pub(crate) field: Ident,
    pub(crate) define: Option<FieldDefine>,
}

impl FromMeta for FieldItem {
    fn from_meta(item: &Meta) -> darling::Result<Self> {
        match item {
            Meta::Path(path) => {
                let Some(field) = path.get_ident().cloned() else {
                    return darling_unknown_format("Ident", &path.span());
                };
                Ok(FieldItem { field, define: None })
            }
            meta @ Meta::List(MetaList { path, .. }) => {
                let Some(field) = path.get_ident().cloned() else {
                    return darling_unknown_format("Ident", &path.span());
                };
                let define = FieldDefine::from_meta(meta)?;
                Ok(FieldItem { field, define: Some(define) })
            }
            Meta::NameValue(pair) => {
                darling_unknown_format("Name Value Meta", &pair.span())
            }
        }
    }

    fn from_string(value: &str) -> darling::Result<Self> {
        let field = syn::parse_str::<Ident>(value)
            ?;
        Ok(Self { field, define: None })
    }
}

#[derive(Debug,Default)]
pub struct Fields(pub(crate) Vec<FieldItem>);

impl Deref for Fields {
    type Target = [FieldItem];

    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}

impl FromMeta for Fields {
    fn from_list(items: &[NestedMeta]) -> darling::Result<Self> {
        from_nest_meta_list(items, Self)
    }
}



#[derive(Debug,Default)]
pub struct SkipFields(pub(crate) Vec<Ident>);

impl FromMeta for SkipFields {
    fn from_list(items: &[NestedMeta]) -> darling::Result<Self> {
        from_nest_meta_list(items, Self)
    }
}

impl Deref for SkipFields {
    type Target = [Ident];

    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}

#[cfg(test)]
mod test {
    use syn::Meta;

    use super::*;

    #[test]
    fn test_field_parse() {
        let tokens = code!(Meta:r#"
fake(
    ty="i32",
    constructor = "|owner:bool, foo:&mut i32, bar:String|parent.foo",
    requires(
        owner,
        foo = "mut",
        bar="owned",
    )
)
        "#);

        let field_item = FieldItem::from_meta(&tokens).expect("Err");
        println!("{field_item:?}");
    }

    #[test]
    fn test_fields_parse() {
        let tokens = code!(Meta:r#"
fields(
    "foo",
    bar,
    fake(
        ty="i32",
        constructor = "|owner:bool, foo:&mut i32, bar:String|parent.foo",
        requires(
            owner,
            foo = "mut",
            bar="owned",
        )
    )
)
        "#);
        let fields = Fields::from_meta(&tokens).expect("Error");
        println!("{fields:?}")
    }

    #[test]
    fn test_skip_fields() {
        let tokens = code!(Meta:r#"
skip_fields(
    "foo",
    "bar",
    "fake"
)
        "#);
        let fields = SkipFields::from_meta(&tokens).expect("Error");
        println!("{fields:?}")
    }
}