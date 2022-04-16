use syn::Ident;

use super::type_mapping::TypeMapping;
use crate::darling_models::{
    utils::{ExtraAttrs, Like, Vis},
    FromIdent,
};

#[derive(Debug, Clone, darling::FromMeta)]
/// A Field Have By One Of Sub Model
pub struct HaveField {
    /// which the field belong to
    #[darling(rename = "for")]
    pub owner: Ident,
    /// the field visibility
    #[darling(default)]
    pub vis: Vis,
    /// wether the field rename to
    /// another in sub model
    #[darling(rename = "rename", default)]
    pub to_name: Option<Ident>,
    /// mapping type from parent to provide  
    /// if necessary
    #[darling(default)]
    pub to_type: Option<TypeMapping>,
    /// extra information which tagging on
    /// the field in SubModel
    #[darling(default)]
    pub extra: ExtraAttrs,
    #[darling(default)]
    pub liking: Option<Like>,
}

impl FromIdent for HaveField {
    /// if HaveField only get a ident
    /// then is the owner
    fn form_ident(ident: syn::Ident) -> Self {
        Self {
            owner: ident,
            vis: Vis::default(),
            extra: ExtraAttrs::default(),
            to_name: None,
            to_type: None,
            liking: None,
        }
    }
}
