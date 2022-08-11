use std::collections::HashMap;

use darling::{FromAttributes, FromDeriveInput, FromField, FromMeta};
use syn::Ident;

use crate::darling_models::{
    field_input::FieldItem,
    struct_input::sub_model_item::SubModels,
    utils::{darling_custom, MetaList, ATTR_NAME},
};

use super::sub_model_item::SubModelHeaderDef;

/// core define information
/// of all SubModels
pub struct SubModelDefs {
    /// the parent Model Name of all SubModel
    pub src_name: Ident,
    /// each subModel head Info
    pub sub_models: HashMap<Ident, SubModelHeaderDef>,
    /// each field in Parent Model
    pub fields: Vec<FieldItem>,
}

impl FromDeriveInput for SubModelDefs {
    fn from_derive_input(input: &syn::DeriveInput) -> darling::Result<Self> {
        let src_name = input.ident.clone();

        // the macro only use for named struct
        let input_fields = match &input.data {
            syn::Data::Struct(s) => &s.fields,
            syn::Data::Enum(_) => Err(darling::Error::unexpected_type("Enum"))?,
            syn::Data::Union(_) => Err(darling::Error::unexpected_type("Union"))?,
        };

        if !input.generics.params.is_empty() {
            darling_custom("SubModel Not Support Generic yet.")?;
        }

        let mut fields = Vec::new();

        for field in input_fields {
            let item = FieldItem::from_field(field)?;
            fields.push(item);
        }

        let meta_list = MetaList::from_attributes(&input.attrs)?
            .filter_with_ident(ATTR_NAME)
            .group_into_nest_meta();

        let sub_models = SubModels::from_list(&meta_list)?.inner;

        Ok(Self {
            src_name,
            sub_models,
            fields,
        })
    }
}
