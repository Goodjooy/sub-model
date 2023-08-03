use std::collections::HashMap;

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
        fields_input: &mut HashMap<Ident, FieldItem>,
        root_model: &Ident,
    ) -> syn::Result<Self> {
        let SubModelHeaderDef { capture_type, data } = header_def;

        let vis = data.vis;
        let extra_attrs = data.extra;
        let ident = data.name;
        let header_fields = data.field;

        let mut fields = Vec::new();

        // load field in header
        for field in header_fields {
            if let Some(item) = fields_input.get_mut(field.def.explicit().as_ref().and_then(|de|de.source.as_ref()).unwrap_or(&field.traget_filed)){
                if let Some(field_def) = SubModelFieldDef::from_field_item(item,&ident,&capture_type)?
                {
                    fields.push(field_def)
                }

            }

        }

        // loading field mapping
        for (_, field) in fields_input {
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

    pub fn from_sub_model_defs(sub_model_defs: SubModelDefs) -> syn::Result<Vec<Self>> {
        let mut vec = Vec::new();
        let mut map = sub_model_defs
            .fields
            .into_iter()
            .map(|field| (field.name.clone(), field))
            .collect();
        for def in sub_model_defs.sub_models.into_values() {
            let def = SubModelDef::from_sub_model_def(
                def,
                &mut map,
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
