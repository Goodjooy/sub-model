use darling::ToTokens;

use syn::Ident;

use crate::bridge::sub_model_def::SubModelDef;

use super::{sub_model_convert_gen::SubModelConvertGen, sub_model_structure::SubModelStructureGen};

pub struct SubModel<'m> {
    model_struct: SubModelStructureGen<'m>,
    convert: SubModelConvertGen<'m>,
}

impl<'m> SubModel<'m> {
    pub fn from_sub_model_def(def: &'m SubModelDef, root_ident: &'m Ident) -> Self {
        Self {
            model_struct: SubModelStructureGen::from_sub_model_def(def),
            convert: SubModelConvertGen::from_sub_model_def(def, root_ident),
        }
    }
}

impl<'m> ToTokens for SubModel<'m> {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let SubModel {
            model_struct,
            convert,
        } = self;

        model_struct.to_tokens(tokens);
        convert.to_tokens(tokens);
    }
}
