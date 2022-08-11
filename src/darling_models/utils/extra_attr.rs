use darling::{FromMeta, ToTokens};
use syn::{Meta, NestedMeta};

/// loading extra attrs from meta
/// it accept all available meta list
/// and not care what it is inside
#[derive(Debug, Clone, Default)]
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

impl ToTokens for ExtraAttrs {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let iter = self.inner.iter();

        let token = quote::quote! {
            #(#[#iter])*
        };
        tokens.extend(token)
    }
}
