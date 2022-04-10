use darling::FromMeta;
use syn::{NestedMeta, Meta};

#[derive(Debug,Clone, Default)]
pub struct ExtraAttrs {
    pub inner: Vec<NestedMeta>,
}

impl FromMeta for ExtraAttrs {
    fn from_list(items: &[NestedMeta]) -> darling::Result<Self> {
        Ok(Self {
            inner: items.to_owned(),
        })
    }

    fn from_nested_meta(item: &NestedMeta) -> darling::Result<Self> {
        Ok(Self {
            inner: vec![item.clone()],
        })
    }

    fn from_string(value: &str) -> darling::Result<Self> {
        let meta = Meta::from_string(value)?;
        Self::from_meta(&meta)
    }
}