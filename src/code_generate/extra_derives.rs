use darling::ToTokens;


pub struct ExtraDerives;

impl ToTokens for ExtraDerives {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        #[cfg(feature="auto_derive_base")]
        tokens.extend(quote::quote! {
            #[derive(Debug, Clone)]
        });

        #[cfg(feature="auto_derive_serde")]
        tokens.extend(quote::quote! {
            #[derive(serde::Serialize,serde::Deserialize)]
        });

        #[cfg(feature="auto_derive_builder")]
        tokens.extend(quote::quote! {
            #[derive(typed_builder::TypedBuilder)]
        })
    }
}