use std::collections::HashMap;

use proc_macro2::Ident;
use syn::{NestedMeta, Type};

use crate::darling_models::utils::Vis;

use super::field_def::FieldDef;

pub struct FieldModelInput {
    /// attrs
    pub attrs: Vec<NestedMeta>,
    pub vis :Vis,
    /// name
    pub name: Ident,
    pub ty: Type,

    pub model_having: HashMap<Ident, FieldDef>,
}
