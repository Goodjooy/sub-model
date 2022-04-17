use darling::{FromMeta, ToTokens};
use syn::Visibility;

use crate::darling_models::utils::darling_unknown_format;

#[derive(Debug, Clone, Default)]
pub struct Vis(Option<Visibility>);

impl From<Visibility> for Vis {
    fn from(v: Visibility) -> Self {
        Self(Some(v))
    }
}

impl FromMeta for Vis {
    fn from_word() -> darling::Result<Self> {
        Ok(Self(Some(Visibility::Inherited)))
    }

    fn from_string(value: &str) -> darling::Result<Self> {
        let vis: Visibility = syn::parse_str(value)?;
        Ok(Self(Some(vis)))
    }

    fn from_value(value: &syn::Lit) -> darling::Result<Self> {
        match value {
            syn::Lit::Str(s) => FromMeta::from_string(&s.value()),
            _ => darling_unknown_format("Not String", value)?,
        }
    }
}

impl ToTokens for Vis {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let quote = match &self.0 {
            Some(vis) => quote::quote! {#vis},
            None => quote::quote! {pub},
        };

        tokens.extend(quote);
    }
}
