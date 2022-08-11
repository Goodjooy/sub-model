use std::collections::HashMap;

use proc_macro2::Ident;

use crate::darling_models::{
    field_input::FieldItem,
    struct_input::SubModelHeaderDef,
    utils::{ExtraAttrs, Vis},
};

use super::sub_model_fields::SubModelFields;

pub struct SubModelGen<'p> {
    pub name: Ident,
    pub parent: &'p Ident,
    pub vis: Vis,
    pub extra: ExtraAttrs,
    pub field: SubModelFields,
}

impl<'p> SubModelGen<'p> {
    pub fn from_sub_model_defs(
        name: Ident,
        parent: &'p Ident,
        sub_model: SubModelHeaderDef,
        fields: &[FieldItem],
    ) -> darling::Result<Self> {
        let SubModelHeaderDef {
            capture_type: ty,
            data,
        } = sub_model;
        let vis = data.vis;
        let extra = data.extra;

        let field =
            SubModelFields::from_fields(&name, &ty, &fields)?.adding_extras(&data.extra_field)?;

        Ok(Self {
            parent,
            vis,
            extra,
            field,
            name,
        })
    }
}

pub fn load_from_sub_model_defs<'怕>(
    src_name: &'怕 Ident,
    sub_models: HashMap<Ident, SubModelHeaderDef>,
    fields: &[FieldItem],
) -> darling::Result<HashMap<Ident, SubModelGen<'怕>>> {
    let mut res = HashMap::with_capacity(sub_models.len());
    for (name, sub) in sub_models {
        let model = SubModelGen::from_sub_model_defs(name, src_name, sub, fields)?;
        res.insert(model.name.clone(), model);
    }
    Ok(res)
}
