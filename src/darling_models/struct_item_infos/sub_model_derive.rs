use std::collections::HashMap;

use darling::{FromAttributes, FromDeriveInput, FromField, FromMeta};
use syn::Ident;

use crate::{
    bridges::{self, LoadingModelInfo},
    darling_models::{
        field_item_infos::{FieldItem, FieldModelInput, FieldWithModelTypeServer},
        struct_item_infos::sub_model_item::SubModels,
        utils::{darling_custom, FieldServer, MetaList, ATTR_NAME},
    },
};

use super::sub_model_item::{self, SubModel};

/// core define information
/// of all SubModels
pub struct SubModelDefs {
    /// the parent Model Name of all SubModel
    pub src_name: Ident,
    /// each subModel head Info
    pub sub_models: HashMap<Ident, SubModel>,
    /// each field in Parent Model
    pub fields: Vec<FieldModelInput>,
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

        if input.generics.params.len() != 0 {
            darling_custom("SubModel Not Support Generic yet.")?;
        }

        let mut fields_mid = Vec::new();

        for field in input_fields {
            let item = FieldItem::from_field(field)?;
            fields_mid.push(item);
        }

        let meta_list = MetaList::from_attributes(&input.attrs)?
            .filter_with_ident(ATTR_NAME)
            .group_into_nest_meta();

        let sub_models = SubModels::from_list(&meta_list)?.inner;

        let mut this = Self {
            src_name,
            sub_models,
            fields: vec![],
        };

        let mut field = Vec::with_capacity(fields_mid.len());

        let map_server = FieldWithModelTypeServer::new(&this);

        for f in fields_mid {
            let f = map_server.proc(f)?;
            field.push(f);
        }

        this.fields = field;
        Ok(this)
    }
}

impl LoadingModelInfo for SubModelDefs {
    fn model_type(&self, model: &Ident) -> Option<bridges::ModelType> {
        self.sub_models.get(model).map(|ty| match &ty.ty {
            &sub_model_item::ModelType::All => bridges::ModelType::All,
            &sub_model_item::ModelType::None => bridges::ModelType::None,
        })
    }

    fn head_ctrl(&self, model: &Ident) -> Option<bool> {
        Some(false)
    }

    type Value = SubModel;

    fn all_models<'s>(&'s self) -> std::collections::hash_map::Keys<'s, Ident, Self::Value> {
        self.sub_models.keys()
    }
}
