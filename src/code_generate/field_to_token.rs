use darling::ToTokens;
use proc_macro2::Ident;

use crate::darling_models::field_input::TypeMapping;

use super::sub_model_fields::ModelField;

impl ToTokens for ModelField {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let token = match self {
            ModelField::Src(src) => {
                let name = src.get_tgt_name();
                let ty = src.get_tgt_type();
                let extra = src.extra_info.extra.inner.iter();

                let vis = src.extra_info.vis.clone();

                quote::quote! {
                    #(
                        #[#extra]
                    )*
                    #vis #name : #ty
                }
            }
            ModelField::Extra(extra) => {
                let name = extra.name.clone();
                let vis = extra.vis.clone();
                let ty = extra.ty.clone();
                let extra = extra.extra.extra.inner.iter();

                quote::quote! {
                    #(
                        #[#extra]
                    )*
                    #vis #name : #ty
                }
            }
        };

        tokens.extend(token);
    }
}

impl ModelField {
    fn to_create_toke(&self, parent: &Ident, tokens: &mut proc_macro2::TokenStream) {
        let token = match self {
            ModelField::Src(src) => {
                let name = src.get_tgt_name();
                let src_name = &src.src_name;

                match &src.extra_info.to_type {
                    Some(TypeMapping { mapping_fun, .. }) => quote::quote! {
                        #name :  #mapping_fun( #parent.#src_name )
                    },
                    None => quote::quote! {
                        #name : #parent.#src_name
                    },
                }
            }
            ModelField::Extra(extra) => {
                let name = &extra.name;
                let create = &extra.create;

                quote::quote! {
                    #name : #create()
                }
            }
        };

        tokens.extend(token);
    }
}

pub struct ModelFieldCreate<'p, 'mf> {
    parent: &'p Ident,
    inner: &'mf ModelField,
}

impl<'p, 'mf> ModelFieldCreate<'p, 'mf> {
    pub fn new(parent: &'p Ident, model: &'mf ModelField) -> Self {
        Self {
            parent,
            inner: model,
        }
    }
}

impl ToTokens for ModelFieldCreate<'_, '_> {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        self.inner.to_create_toke(self.parent, tokens)
    }
}
