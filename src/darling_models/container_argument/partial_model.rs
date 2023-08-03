//! define the partical model infos
//!
//!```
//! #[partial(
//!    Bar(
//!        vis="pub",
//!        none_fields,
//!        fields(
//!            foo,
//!            bar,
//!            // extra fields define
//!            fake(
//!                ty="i32",
//!                constructor = "|owner:bool, foo:&mut i32, bar:String|parent.foo",
//!                requires(
//!                    owner,
//!                    foo = "mut",
//!                    bar="owned",
//!                )
//!            )
//!        ),
//!        // extra meta define
//!         meta(derive(Debug))
//!    )
//! )]
//! struct Foo{
//!     foo:i32,
//!     bar:String,
//!     owner : bool
//! }
//!```

use darling::FromMeta;
use proc_macro2::Ident;
use syn::{Meta, MetaList};
use syn::spanned::Spanned;

use crate::darling_models::container_argument::partial_model::field_define::SkipFields;
use crate::darling_models::utils::{darling_custom, darling_unknown_format, ExtraAttrs, Vis};

mod field_define;

#[derive(Debug)]
struct PartialModel {
    name: Ident,
    define: PartialModelDefine,
}

impl FromMeta for PartialModel {
    fn from_meta(item: &Meta) -> darling::Result<Self> {
        match item {
            meta @ Meta::List(MetaList { path, .. }) => {
                let Some(name) = path.get_ident().cloned() else {
                    return darling_unknown_format("Ident", &item.span());
                };
                let define = PartialModelDefine::from_meta(meta)?;
                Ok(Self { name, define })
            }
            meta => {
                darling_custom("Expect internal define")
                .map_err(|e| e.with_span(&meta.span()))
            }
        }
    }
}

#[derive(FromMeta, Debug)]
#[darling(and_then = "Self::valid")]
struct PartialModelDefine {
    #[darling(default)]
    vis: Vis,
    #[darling(default)]
    all_fields: bool,
    #[darling(default)]
    none_fields: bool,
    #[darling(default)]
    fields: field_define::Fields,
    #[darling(default)]
    skip_fields: SkipFields,
    #[darling(default)]
    meta: ExtraAttrs,
}

impl PartialModelDefine {
    fn valid(mut self) -> darling::Result<Self> {
        match (self.all_fields, self.none_fields) {
            (true, true) => { return darling_custom("`all_field` and `none_field` can only select one"); }
            (false, false) => { self.none_fields = true; }
            _ => {}
        };

        Ok(self)
    }
}

#[cfg(test)]
mod test {
    use darling::FromMeta;
    use syn::Meta;

    use crate::darling_models::container_argument::partial_model::{PartialModel, PartialModelDefine};

    #[test]
    fn test_model_parse() {
        let token = code!(Meta:r#"
Bar(
    vis="pub",none_fields,
    fields(
        foo,
        bar,
        // extra fields define
        fake(
            ty="i32",
            constructor = "|owner:bool, foo:&mut i32, bar:String|parent.foo",
            requires(
                owner,
                foo = "mut",
                bar="owned",
            )
        )
    ),
    // extra meta define
    meta(derive(Debug))
)
        "#);

        let ret = PartialModel::from_meta(&token).expect("ERROR");

        println!("{ret:#?}");
    }
}