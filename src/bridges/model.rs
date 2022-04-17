use std::collections::hash_map;

use proc_macro2::Ident;

pub trait LoadingModelInfo {
    /// get the model field capture type
    fn model_type(&self, model: &Ident) -> Option<ModelType>;
    /// the head has `include` or `exclude`
    /// the field should not have any tagged
    fn head_ctrl(&self, model: &Ident) -> Option<bool>;

    type Value;
    /// get all model keys 
    fn all_models<'s>(&'s self) -> hash_map::Keys<'s, Ident, Self::Value>;
}



#[derive(Debug)]
pub enum ModelType {
    All,
    None,
}
