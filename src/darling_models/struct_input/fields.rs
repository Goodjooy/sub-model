use std::marker::PhantomData;

use darling::{util::Override, FromMeta};
use syn::{Ident, MetaList};
use syn::spanned::Spanned;

use crate::darling_models::{
    field_input::TypeMapping,
    utils::{darling_custom, path_to_ident::path_to_ident},
};

#[derive(Debug, FromMeta)]
pub struct FieldSelectInner {
    /// 原始field, 默认为同名field
    pub source: Option<Ident>,
    /// 类型映射
    pub type_mapping: Option<TypeMapping>,
}

#[derive(Debug)]
pub struct FieldSelect {
    /// 目标field
    pub traget_filed: Ident,
    /// 更多配置
    pub def: Override<FieldSelectInner>,
}

impl FieldSelect {
    pub fn check_none_mod(self) -> darling::Result<Self> {
        if let Override::Explicit(..) = self.def {
            darling_custom("`none` field capture mod cannot set extra field info")
                .map_err(|err| err.with_span(&self.traget_filed.span()))
        } else {
            Ok(self)
        }
    }
}

impl FromMeta for FieldSelect {
    fn from_meta(item: &syn::Meta) -> darling::Result<Self> {
        match item {
            syn::Meta::Path(path) => Ok(Self {
                traget_filed: path_to_ident(path)?,
                def: Override::Inherit,
            }),
            syn::Meta::List(MetaList { path, nested, .. }) => Ok(Self {
                traget_filed: path_to_ident(path)?,
                def: Override::from_list(nested.iter().cloned().collect::<Vec<_>>().as_slice())?,
            }),
            syn::Meta::NameValue(kv) => Self::from_value(&kv.lit),
        }
    }
}
