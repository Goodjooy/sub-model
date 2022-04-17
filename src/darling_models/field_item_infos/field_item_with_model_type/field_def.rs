use proc_macro2::Ident;
use syn::{NestedMeta, Path, Type};

use crate::darling_models::{
    field_item_infos::field_types_init::HaveField,
    utils::{Like, Vis},
};

#[derive(Debug, Default)]
pub struct FieldDef {
    vis: Vis,
    rename: Option<Ident>,
    type_map: Option<(Type, Path)>,
    extra: Vec<NestedMeta>,
    like: Option<Like>,
}

impl From<HaveField> for FieldDef {
    fn from(
        HaveField {
            owner,
            vis,
            to_name,
            to_type,
            extra,
            liking,
        }: HaveField,
    ) -> Self {
        Self {
            vis,
            rename: to_name,
            type_map: to_type.map(|v| (v.target_type, v.mapping_fun)),
            extra: extra.inner,
            like: liking,
        }
    }
}
