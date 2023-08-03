use darling::FromMeta;
use proc_macro2::Ident;
use syn::spanned::Spanned;
use syn::{Lit, Meta, NestedMeta};
use tap::{Tap, TapFallible};

use crate::darling_models::utils::{darling_custom, darling_unknown_format, from_nest_meta_list};

#[derive(Default, Debug, Eq, PartialEq)]
pub enum RequireMode {
    #[default]
    Ref,
    MutRef,
    Owner,
}

#[derive(Debug)]
pub struct RequireField {
    pub(crate) field: Ident,
    pub(crate) mode: RequireMode,
}

impl FromMeta for RequireField {
    fn from_meta(item: &Meta) -> darling::Result<Self> {
        match item {
            Meta::Path(path) => {
                let Some(ident) = path.get_ident()else {
                    return darling_unknown_format("ident", &path.span());
                };
                Ok(Self {
                    field: ident.to_owned(),
                    mode: RequireMode::Ref,
                })
            }
            Meta::List(span) => darling_custom("Expect K-V pair or Key only, but get List")
                .map_err(|e| e.with_span(&span.span())),
            Meta::NameValue(kv) => {
                let Some(ident) = kv.path.get_ident().cloned() else {
                    return darling_unknown_format("ident", &kv.path.span());
                };
                let Lit::Str(s) = &kv.lit else {
                    return darling_unknown_format("string literal", &kv.lit.span());
                };
                let s = s.value();

                let mode = match s.as_str() {
                    "mut" => RequireMode::MutRef,
                    "owned" => RequireMode::Owner,
                    "ref" => RequireMode::Ref,
                    _ => {
                        return darling_custom(&format!(
                            "Expect `mut`, `owned`,`ref` but get {}",
                            s
                        ))
                        .map_err(|e| e.with_span(&kv.lit.span()))
                    }
                };

                Ok(Self { field: ident, mode })
            }
        }
    }

    fn from_string(value: &str) -> darling::Result<Self> {
        let field = syn::parse_str::<Ident>(value)?;
        Ok(Self {
            field,
            mode: Default::default(),
        })
    }
}

#[derive(Debug)]
pub struct RequireList(pub(crate) Vec<RequireField>);

impl FromMeta for RequireList {
    fn from_list(items: &[NestedMeta]) -> darling::Result<Self> {
        from_nest_meta_list(items, Self)
    }
}

#[cfg(test)]
mod test {
    use darling::FromMeta;
    use quote::format_ident;
    use syn::Meta;

    use crate::darling_models::container_argument::partial_model::field_define::requires::{
        RequireList, RequireMode,
    };

    #[test]
    fn test_field_requires() {
        let token = code!(r#"
requires(
    "owner",
    refs = "ref",
    foo = "mut",
    bar = "owned",)"# => Meta);

        let ret = RequireList::from_meta(&token).expect("Error load data");
        println!("{ret:?}");

        assert_eq!(ret.0.len(), 4);

        assert_eq!(ret.0[0].field, format_ident!("owner"));
        assert_eq!(ret.0[0].mode, RequireMode::Ref);

        assert_eq!(ret.0[1].field, format_ident!("refs"));
        assert_eq!(ret.0[1].mode, RequireMode::Ref);

        assert_eq!(ret.0[2].field, format_ident!("foo"));
        assert_eq!(ret.0[2].mode, RequireMode::MutRef);

        assert_eq!(ret.0[3].field, format_ident!("bar"));
        assert_eq!(ret.0[3].mode, RequireMode::Owner);
    }
}
