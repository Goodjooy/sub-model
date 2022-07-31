#![allow(clippy::large_enum_variant)]
mod code_generate;

use code_generate::sub_model::load_from_sub_model_defs;
use darling::FromDeriveInput;
use darling_models::struct_item_infos::SubModelDefs;
use syn::{parse_macro_input, DeriveInput};

mod darling_models;

#[doc= include_str!("../readme.md")]
#[proc_macro_derive(SubModel, attributes(sub_model))]
pub fn sub_model_derive_marco(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let derive_input = parse_macro_input!(input as DeriveInput);

    let SubModelDefs {
        src_name,
        sub_models,
        fields,
    } = match <SubModelDefs as FromDeriveInput>::from_derive_input(&derive_input) {
        Ok(d) => d,
        Err(err) => return err.write_errors().into(),
    };

    let sub_models = load_from_sub_model_defs(&src_name, sub_models, &fields)
        .expect("Darling Error")
        .into_iter()
        .map(|(_, v)| v);

    quote::quote! {
        #(
            #sub_models
        )*

    }
    .into()
}
