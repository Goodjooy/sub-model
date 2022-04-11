use std::{
    ops::{Add, AddAssign, SubAssign},
    time::Duration,
};

use syn::{parse_macro_input, DeriveInput};

mod darling_models;

#[proc_macro_derive(SubModel, attributes(sub_model))]
pub fn sub_model_derive_marco(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let _derive_input = parse_macro_input!(input as DeriveInput);

    quote::quote! {}.into()
}
