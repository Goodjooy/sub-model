use std::collections::HashMap;

use proc_macro2::Ident;
use syn::{NestedMeta, Type};

use crate::{bridges::FieldBaseInfo, darling_models::utils::Vis};

use super::field_def::FieldDef;

/// each field with base info
/// and sub model field info
pub struct FieldModelInput {
    /// attrs
    pub attrs: Vec<NestedMeta>,
    /// the visibility
    pub vis: Vis,
    /// name
    pub name: Ident,
    /// the field type
    pub ty: Type,

    /// sub model info
    /// Key => Sub Model Name
    /// Value => Field In Sub Model
    /// if Key not exist, Target SubModel Do Not Want this field
    pub model_having: HashMap<Ident, FieldDef>,
}

impl FieldBaseInfo for FieldModelInput {


    fn vis(&self) -> &Vis {
        &self.vis
    }

    fn name(&self) -> &Ident {
        &self.name
    }

    fn ty(&self) -> &syn::Type {
        &self.ty
    }
}
