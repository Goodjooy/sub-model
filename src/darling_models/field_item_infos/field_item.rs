use std::collections::HashMap;

use darling::{FromAttributes, FromField, FromMeta};
use syn::{Ident, Type};

use crate::darling_models::utils::{darling_duplicate_field, MetaList, ATTR_NAME};

use super::field_marco::{FieldMarcos, FieldType};

pub struct FieldItem {
    pub name: Ident,
    pub ty: Type,
    pub sub_models: HashMap<Ident, FieldType>,
}

impl FromField for FieldItem {
    fn from_field(field: &syn::Field) -> darling::Result<Self> {
        let syn::Field {
            attrs, ident, ty, ..
        } = field;
        let ty = ty.clone();
        let ident: Option<&Ident> = ident.into();
        let name = ident
            .cloned()
            .ok_or(darling::Error::unsupported_format("UnNamedField").with_span(field))?;

        let meta_list = MetaList::from_attributes(attrs)?
            .filter_with_ident(ATTR_NAME)
            .group_into_nest_meta();

        let sub_models = FieldMarcos::from_list(&meta_list)?.inner;

        let mut sub_maps = HashMap::with_capacity(sub_models.len());

        for model in sub_models {
            let name = model.get_owner();
            if let Some(_) = sub_maps.insert(name.clone(), model) {
                darling_duplicate_field(&name)?;
            }
        }

        Ok(Self {
            name,
            ty,
            sub_models: sub_maps,
        })
    }
}
