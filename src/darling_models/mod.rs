mod commons;
use darling::FromMeta;

#[macro_use]
pub mod utils;
mod container_argument;
pub mod field_input;
pub mod struct_input;

pub trait FromIdent: FromMeta {
    fn form_ident(ident: syn::Ident) -> Self;
}
