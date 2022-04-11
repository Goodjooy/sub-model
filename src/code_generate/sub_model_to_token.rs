use darling::ToTokens;

use quote::format_ident;

use super::{field_to_token::ModelFieldCreate, sub_model::SubModelGen};

impl ToTokens for SubModelGen<'_> {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let name = &self.name;
        let parent = &self.parent;
        let v_parent = format_ident!("__parent");
        let vis = &self.vis;
        let extra = self.extra.inner.iter();
        let fields = self.field.inner.iter().map(|(_, v)| v);

        let def_token = quote::quote! {
            #(
                #[#extra]
            )*
            #vis struct #name {
                #(
                    #fields
                ),*
            }
        };

        tokens.extend(def_token);

        let create_field = self
            .field
            .inner
            .iter()
            .map(|(_, v)| ModelFieldCreate::new(&v_parent, v));

        let convert_toke = quote::quote! {
            impl From<#parent> for #name{
                fn from(#v_parent : #parent)->Self{
                    Self{
                        #(
                            #create_field
                        ),*
                    }
                }
            }
        };

        tokens.extend(convert_toke);
    }
}
