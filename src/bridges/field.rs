use syn::{Ident, NestedMeta};

use crate::darling_models::utils::{Like, Vis};

use super::mid_model::field_construct::FieldValueFrom;

pub trait FieldBaseInfo {
    fn vis(&self) -> &Vis;
    fn name(&self) -> &Ident;
    fn ty(&self) -> &syn::Type;
}

pub trait LikeTo {
    fn like_to(&self) -> Option<&Like>;
}

pub trait FieldConstruct {
    fn field_value_from(&self) -> FieldValueFrom;
}

pub trait FieldExtras {
    fn field_extra(&self) -> &[NestedMeta];
}

pub trait FieldInfo
where
    Self: FieldBaseInfo + FieldConstruct + FieldExtras,
{
}

pub trait LikeLoader {
    type Super: FieldInfo;
    /// get target field info that comes from super
    fn get_super(&self, field: &Ident) -> &Self::Super;

    type ModelField: FieldInfo;
    /// get target field info from special model
    fn get_model_field(&self, model: &Ident, field: &Ident) -> &Self::Super;
}
