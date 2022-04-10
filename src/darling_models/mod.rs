use darling::FromMeta;

#[macro_use]
mod utils;
mod field_item_infos;
mod struct_item_infos;

pub trait FromIdent: FromMeta {
    fn form_ident(ident: syn::Ident) -> Self;
}
