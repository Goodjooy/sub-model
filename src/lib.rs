#![allow(clippy::large_enum_variant)]

#[macro_use]
mod utils;
mod bridge;
mod code_generate;

use code_generate::code_generate;
use darling::FromDeriveInput;
use darling_models::struct_input::SubModelDefs;
use syn::{parse_macro_input, DeriveInput};

mod darling_models;

#[doc= include_str!("../readme.md")]
#[proc_macro_derive(SubModel, attributes(sub_model))]
pub fn sub_model_derive_marco(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let derive_input = parse_macro_input!(input as DeriveInput);

    let sub_model_defs = darling_err!(<SubModelDefs as FromDeriveInput>::from_derive_input(
        &derive_input
    ));

    code_generate(sub_model_defs)
}
