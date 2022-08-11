use darling::ToTokens;
use syn::Ident;

use crate::bridge::{sub_model_def::SubModelDef, sub_model_field_def::SubModelFieldDef};

use super::sub_model_field_construct_gen::SubModelFieldConstructGen;

pub struct SubModelConvertGen<'m> {
    root_model: &'m Ident,
    name: &'m Ident,
    root_ident: &'m Ident,
    fields: &'m [SubModelFieldDef],
}

impl<'m> SubModelConvertGen<'m> {
    pub fn from_sub_model_def(def: &'m SubModelDef, root_ident: &'m Ident) -> Self {
        Self {
            root_model: &def.root_model,
            name: &def.ident,
            fields: def.fields.as_slice(),
            root_ident,
        }
    }
}

impl<'m> ToTokens for SubModelConvertGen<'m> {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let SubModelConvertGen {
            root_model,
            name,
            fields,
            root_ident,
        } = self;
        let fields = fields
            .into_iter()
            .map(|v| SubModelFieldConstructGen::from_sub_model_field_def(v, &root_ident));

        tokens.extend(quote::quote! {
            impl std::convert::From < #root_model > for #name {
                fn from ( #root_ident : # root_model ) -> Self{
                    Self{
                        #(
                            #fields
                        )*
                    }
                }
            }
        })
    }
}
