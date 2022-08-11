use proc_macro::TokenStream;
use quote::format_ident;

use crate::{bridge::sub_model_def::SubModelDef, darling_models::struct_input::SubModelDefs};

use self::sub_model::SubModel;

mod extra_derives;
mod sub_model;
mod sub_model_convert_gen;
mod sub_model_field_construct_gen;
mod sub_model_field_gen;
mod sub_model_structure;

pub fn code_generate(input: SubModelDefs) -> TokenStream {
    let sub_models = syn_err!(SubModelDef::from_sub_model_defs(input));
    let root_ident = format_ident!("__parent");
    let iter = sub_models
        .iter()
        .map(|def| SubModel::from_sub_model_def(def, &root_ident));

    quote::quote! {
        #(
            #iter
        )*
    }
    .into()
}
