use darling::FromMeta;
use syn::Ident;

use crate::darling_models::FromIdent;

#[derive(Debug, FromMeta)]
/// the field ignore by a SubModel
/// **only can apply on ALL**
pub struct IgnoreField {
    #[darling(rename = "for")]
    /// the SubModel Want to ignore this field
    pub owner: Ident,
}

impl FromIdent for IgnoreField {
    fn form_ident(ident: syn::Ident) -> Self {
        Self { owner: ident }
    }
}
