use darling::FromMeta;

#[macro_use]
pub mod utils;
pub mod field_item_infos;
pub mod struct_item_infos;

pub trait FromIdent: FromMeta {
    fn form_ident(ident: syn::Ident) -> Self;
}
