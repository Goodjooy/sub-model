use syn::Ident;

use crate::darling_models::{
    field_input::FieldItem,
    struct_input::{SubModelDefs, SubModelHeaderDef},
    utils::{ExtraAttrs, Vis},
};

use super::sub_model_field_def::SubModelFieldDef;

#[derive(Debug)]
pub struct SubModelDef {
    pub vis: Vis,
    /// the name of root model
    pub root_model: Ident,
    /// the name of sub model
    pub ident: Ident,
    /// extra attrs
    pub extra_attrs: ExtraAttrs,

    pub fields: Vec<SubModelFieldDef>,
}

impl SubModelDef {
    pub fn from_sub_model_def(
        header_def: SubModelHeaderDef,
        fields_input: &mut Vec<FieldItem>,
        root_model: &Ident,
    ) -> syn::Result<Self> {
        let SubModelHeaderDef { capture_type, data } = header_def;

        let vis = data.vis;
        let extra_attrs = data.extra;
        let ident = data.name;

        let mut fields = Vec::new();
        // loading field mapping
        for field in fields_input {
            let field_def = SubModelFieldDef::from_field_item(field, &ident, &capture_type)?;
            if let Some(field_def) = field_def {
                fields.push(field_def);
            }
        }

        // load extra fields
        for (_, def) in data.extra_field.inner {
            let field_def = SubModelFieldDef::from_extra_field(def)?;
            fields.push(field_def);
        }

        Ok(Self {
            vis,
            root_model: root_model.to_owned(),
            ident,
            extra_attrs,
            fields,
        })
    }

    pub fn from_sub_model_defs(mut sub_model_defs: SubModelDefs) -> syn::Result<Vec<Self>> {
        let mut vec = Vec::new();
        for def in sub_model_defs.sub_models.into_values() {
            let def = SubModelDef::from_sub_model_def(
                def,
                &mut sub_model_defs.fields,
                &sub_model_defs.src_name,
            )?;
            vec.push(def);
        }

        Ok(vec)
    }
}

#[cfg(test)]
mod test {
    use darling::FromDeriveInput;
    use syn::DeriveInput;

    use crate::darling_models::struct_input::SubModelDefs;

    use super::SubModelDef;

    #[test]
    fn test() {
        let tokens: DeriveInput = syn::parse_str(
            r#"
            #[sub_model(
                all(
                    vis = "pub",
                    name = "Foo",
                    extra(
                        derive(Copy),
                        doc="acca"
                    ),
                    extra_field(
                        a(ty="i32",from="Default::default"),
                        c(ty="u8",from="Default::default")
                    ),
                ),
            )]
            pub struct Bar{
                #[sub_model(
                    having(
                        for = "Foo",
                        to_type(ty = "String", by= "Path::to::I322String")
                    )
                )]
                foo : i32,
                #[sub_model(
                    ignore("Foo")
                )]
                bar : String,
                foo_bar: u32,
            }
        "#,
        )
        .unwrap();

        let v = SubModelDefs::from_derive_input(&tokens).unwrap();

        let v = SubModelDef::from_sub_model_defs(v)
            .unwrap()
            .into_iter()
            .next()
            .unwrap();

        assert_eq!(v.fields.len(), 4);
        println!("{v:#?}")
    }
}
