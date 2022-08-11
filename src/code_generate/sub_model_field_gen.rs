use darling::ToTokens;
use syn::{Ident, Type};

use crate::{
    bridge::sub_model_field_def::{FieldFromRootTypeMapping, SubModelField, SubModelFieldDef},
    darling_models::utils::{ExtraAttrs, Vis},
};

pub struct SubModelFieldGen<'r> {
    extra_attrs: &'r ExtraAttrs,
    vis: &'r Vis,
    ident: &'r Ident,
    ty: &'r Type,
}

impl<'r> SubModelFieldGen<'r> {
    pub fn from_sub_model_field_def(def: &'r SubModelFieldDef) -> Self {
        let (ident, ty) = match &def.define {
            SubModelField::FromRoot(fr) => (
                fr.filed_ident_mapping.as_ref().unwrap_or(&fr.root_name),
                match &fr.field_type_mapping {
                    FieldFromRootTypeMapping::Direct => &fr.root_type,
                    FieldFromRootTypeMapping::MappingType { map_ty, .. } => map_ty,
                },
            ),
            SubModelField::Extra(ex) => (&ex.name, &ex.ty),
        };
        Self {
            extra_attrs: &def.extra,
            vis: &def.vis,
            ident,
            ty,
        }
    }
}

impl<'r> ToTokens for SubModelFieldGen<'r> {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let SubModelFieldGen {
            extra_attrs,
            vis,
            ident,
            ty,
        } = self;

        tokens.extend(quote::quote! {
            #extra_attrs
            #vis #ident : #ty,
        })
    }
}
