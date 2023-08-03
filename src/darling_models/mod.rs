mod commons;
use darling::FromMeta;

#[macro_use]
pub mod utils;
pub mod field_input;
pub mod struct_input;
mod container_argument;

pub trait FromIdent: FromMeta {
    fn form_ident(ident: syn::Ident) -> Self;
}
