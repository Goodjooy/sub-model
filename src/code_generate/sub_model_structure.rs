use darling::ToTokens;
use syn::Ident;

use crate::{
    bridge::{sub_model_def::SubModelDef, sub_model_field_def::SubModelFieldDef},
    darling_models::utils::{ExtraAttrs, Vis},
};

use super::{extra_derives::ExtraDerives, sub_model_field_gen::SubModelFieldGen};

pub struct SubModelStructureGen<'m> {
    vis: &'m Vis,
    name: &'m Ident,
    extra_attrs: &'m ExtraAttrs,
    fields: &'m [SubModelFieldDef],
}

impl<'m> SubModelStructureGen<'m> {
    pub fn from_sub_model_def(sub_model_def: &'m SubModelDef) -> Self {
        Self {
            vis: &sub_model_def.vis,
            name: &sub_model_def.ident,
            extra_attrs: &sub_model_def.extra_attrs,
            fields: sub_model_def.fields.as_slice(),
        }
    }
}

impl<'m> ToTokens for SubModelStructureGen<'m> {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let SubModelStructureGen {
            vis,
            name,
            extra_attrs,
            fields,
        } = self;
        let fields = fields
            .into_iter()
            .map(SubModelFieldGen::from_sub_model_field_def);
        let extra_derive = ExtraDerives;
        let token = quote::quote! {
            #extra_derive
            #extra_attrs
            #vis struct #name {
                #(
                    #fields
                )*
            }
        };

        tokens.extend(token)
    }
}
