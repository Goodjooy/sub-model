use darling::ToTokens;
use syn::{Ident, Path};

use crate::bridge::sub_model_field_def::{
    FieldFromRootTypeMapping, SubModelField, SubModelFieldDef,
};

pub enum Map<'s> {
    FromRoot {
        ident_from: Option<&'s Ident>,
        type_from: Option<&'s Path>,
    },
    Create(&'s Path),
}

pub struct SubModelFieldConstructGen<'s> {
    name: &'s Ident,
    root_model_ident: &'s Ident,
    map: Map<'s>,
}

impl<'s> SubModelFieldConstructGen<'s> {
    pub fn from_sub_model_field_def(def: &'s SubModelFieldDef, root_ident: &'s Ident) -> Self {
        match &def.define {
            SubModelField::FromRoot(fr) => Self {
                // if ident map set ,name is ident else name is root field name
                name: fr.filed_ident_mapping.as_ref().unwrap_or(&fr.root_name),
                root_model_ident: root_ident,
                map: Map::FromRoot {
                    ident_from: fr.filed_ident_mapping.as_ref().map(|_| &fr.root_name),
                    type_from: match fr.field_type_mapping {
                        FieldFromRootTypeMapping::Direct => None,
                        FieldFromRootTypeMapping::MappingType { ref mapper, .. } => Some(mapper),
                    },
                },
            },
            SubModelField::Extra(ex) => Self {
                name: &ex.name,
                root_model_ident: root_ident,
                map: Map::Create(&ex.constructor),
            },
        }
    }
}

impl<'s> ToTokens for SubModelFieldConstructGen<'s> {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let SubModelFieldConstructGen {
            name,
            root_model_ident,
            map,
        } = self;
        let val = match map {
            Map::FromRoot {
                ident_from,
                type_from,
            } => {
                match (ident_from, type_from) {
                    // no map type and map name
                    (None, None) => quote::quote! { #root_model_ident . #name },
                    // map type
                    (None, Some(mapper)) => quote::quote! {#mapper ( #root_model_ident . #name ) },
                    // map ident
                    (Some(ident), None) => quote::quote! { #root_model_ident. #ident },
                    (Some(ident), Some(mapper)) => {
                        quote::quote! {#mapper ( root_model_ident . #ident )}
                    }
                }
            }
            Map::Create(constructor) => quote::quote! { #constructor () },
        };

        tokens.extend(quote::quote! {
            #name : #val,
        })
    }
}
