use std::collections::HashMap;

use darling::{FromAttributes, FromField, FromMeta};
use syn::{Ident, Type};

use crate::darling_models::utils::{darling_duplicate_field, MetaList, ATTR_NAME};

use super::field_marco::{FieldMarcos, FieldType};

/// the needing field info from Parent Model field 
pub struct FieldItem {
    /// the name of Parent field
    pub name: Ident,
    /// the type of Parent field
    pub ty: Type,
    /// all SubModel relate to this field
    pub sub_models: HashMap<Ident, FieldType>,
}

impl FromField for FieldItem {
    fn from_field(field: &syn::Field) -> darling::Result<Self> {
        let syn::Field {
            attrs, ident, ty, ..
        } = field;
        let ty = ty.clone();
        let ident: Option<&Ident> = ident.into();
        // if is the `Tuple Struct` the ident will be `None`
        // the `SubModel` only support to NamedStruct
        let name = ident
            .cloned()
            .ok_or(darling::Error::unsupported_format("UnNamedField").with_span(field))?;

        // loading Vec<nested meta> from field Attr
        // under specify attr name
        let meta_list = MetaList::from_attributes(attrs)?
            .filter_with_ident(ATTR_NAME)
            .group_into_nest_meta();

        let sub_models = FieldMarcos::from_list(&meta_list)?.inner;

        let mut sub_maps = HashMap::with_capacity(sub_models.len());
        // load all sub model info into HashMap
        // and check whether duplicate field or not
        for (name,model) in sub_models {
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
