use std::collections::HashMap;

use darling::FromMeta;
use syn::Ident;

use crate::darling_models::{
    utils::{darling_duplicate_field, load_from_meta_list, only_neat_meta_list, ExtraAttrs, Vis},
    FromIdent,
};

use super::extra_fields::ExtraFields;

#[derive(Debug, FromMeta)]
/// each SubModel Define On
/// the struct Item Def
pub struct SubModelItem {
    /// the name of the Sub Model
    pub name: Ident,
    /// the visibility of the
    /// Sub Model
    #[darling(default)]
    pub vis: Vis,
    /// the extra Tagging Meta
    /// on the SubModel
    #[darling(default)]
    pub extra: ExtraAttrs,
    /// the Extra Fields That *Not*
    /// in Parent Fields
    #[darling(default)]
    pub extra_field: ExtraFields,
}

impl FromIdent for SubModelItem {
    fn form_ident(ident: syn::Ident) -> Self {
        Self {
            vis: Default::default(),
            name: ident,
            extra: Default::default(),
            extra_field: Default::default(),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum ModelType {
    /// default will capture all field
    /// of parent Model except the fields
    /// which tagged by `ignore`
    All,
    /// default will NOT capture all field
    /// of parent Model except the fields
    /// which tagged by `want`
    None,
}

/// full base define of sub Model
#[derive(Debug)]
pub struct SubModel {
    /// the sub Model field capture behaver
    pub ty: ModelType,
    /// the detail of the sub model defined
    pub data: SubModelItem,
}

impl SubModel {
    fn get_name(&self) -> Ident {
        self.data.name.clone()
    }
}

impl FromMeta for SubModel {
    fn from_nested_meta(item: &syn::NestedMeta) -> darling::Result<Self> {
        // the expect item position
        // #[sub_model(all("MockA"))]
        //             ^^^^^^^^^^^^
        // inner           ^^^^^^
        //
        // -----
        //
        // lit like this
        // #[sub_model("Mock")]
        // not accept
        //
        // -----
        //
        // Meta::Path like this
        // #[sub_model(all)]
        // not accept
        //
        // ----
        //
        // Meta::NameValue like this
        // #[sub_model(all="Mock")]
        // not accept
        only_neat_meta_list(item, |path, meta_list| {
            Ok(
                // handle when #[sub_model(all("Mock"))]
                if path.is_ident("all") {
                    SubModel {
                        ty: ModelType::All,
                        data: load_from_meta_list(&meta_list)?,
                    }
                }
                // handle when #[sub_model(none("Mock"))]
                else if path.is_ident("none") {
                    SubModel {
                        ty: ModelType::None,
                        data: load_from_meta_list(&meta_list)?,
                    }
                } else {
                    Err(darling::Error::unknown_field_path(path))?
                },
            )
        })
    }
}

/// a group of SubModel
/// load all SubModel Header Def
pub struct SubModels {
    pub inner: HashMap<Ident, SubModel>,
}

impl FromMeta for SubModels {
    fn from_list(items: &[syn::NestedMeta]) -> darling::Result<Self> {
        let mut inner = HashMap::with_capacity(items.len());
        for item in items {
            let model = SubModel::from_nested_meta(item)?;
            // if the target key exist , the HashMap::insert will return Some()
            // return the exist value
            // when the return is Some , that means we have 2 same name SubModel
            // SubModel can NOT have same name
            if let Some(ref m) = inner.insert(model.get_name(), model) {
                darling_duplicate_field(&m.get_name())?;
            }
        }

        Ok(Self { inner })
    }
}

#[cfg(test)]
mod test {
    use darling::FromMeta;
    use proc_macro2::Ident;
    use syn::NestedMeta;

    use crate::darling_models::struct_item_infos::ModelType;

    use super::{SubModel, SubModels};

    #[test]
    fn one_with_only_name() {
        let all = code!(
            r#"all("AllMock")"#=>NestedMeta
        );

        let all_ident = code!("AllMock"=>Ident);

        let none = code!(
            r#"none("NoneMock")"#=>NestedMeta
        );

        let none_ident = code!("NoneMock"=>Ident);

        let all_resp = <SubModel as FromMeta>::from_nested_meta(&all).expect("cannot Load");

        assert_eq!(all_resp.ty, ModelType::All);
        assert_eq!(all_resp.get_name(), all_ident);

        println!("all out : {:?}", all_resp);

        let none_resp = <SubModel as FromMeta>::from_nested_meta(&none).unwrap();

        assert_eq!(none_resp.ty, ModelType::None);
        assert_eq!(none_resp.get_name(), none_ident);

        println!("none out : {:?}", none_resp);
    }

    #[test]
    fn one_with_extra_infos() {
        let all = code!(
            NestedMeta:r#"all(
                extra(
                    derive(Copy),
                    doc="acca"
                ),
                extra_field(
                    a(ty="i32",from="Default::default"),
                    c(ty="u8",from="Default::default")
                ),
                name = "MockAll",
                vis = "pub"
            )
            "#
        );
        let ident = code!(Ident:"MockAll");

        let all_resp = <SubModel as FromMeta>::from_nested_meta(&all).unwrap();

        println!("out {:?}", all_resp);

        assert_eq!(all_resp.ty, ModelType::All);
        assert_eq!(all_resp.get_name(), ident);
        assert_eq!(all_resp.data.extra.inner.len(), 2);
        assert_eq!(all_resp.data.extra_field.inner.len(), 2);
    }

    #[test]
    fn multi_mix() {
        let item = code!(
            NestedMeta:r#"
            mock(
                all("MockA"),
                none("MockB"),
                all(
                    name="MockC",
                    vis="",
                    extra(
                        derive(Clone),
                        doc = "abcde"
                    ),
                    extra_field(
                        a(ty="i32",from="i32_def"),
                        c(ty="u8",from="u8_def"),
                    )
                )
            )
            "#
        );
        let mock_a = code!(Ident:"MockA");
        let mock_b = code!(Ident:"MockB");
        let mock_c = code!(Ident:"MockC");

        let code_out = <SubModels as FromMeta>::from_nested_meta(&item).unwrap().inner;

        
        assert!(code_out.get(&mock_a).is_some());
        assert_eq!(code_out.get(&mock_a).unwrap().get_name(),mock_a);
        assert_eq!(code_out.get(&mock_a).unwrap().ty,ModelType::All);
        assert_eq!(code_out.get(&mock_a).unwrap().data.extra_field.inner.len(),0);
        assert_eq!(code_out.get(&mock_a).unwrap().data.extra.inner.len(),0);

        println!("out {:?}\n\n",code_out.get(&mock_a).unwrap());
        
        
        assert!(code_out.get(&mock_b).is_some());
        assert_eq!(code_out.get(&mock_b).unwrap().get_name(),mock_b);
        assert_eq!(code_out.get(&mock_b).unwrap().ty,ModelType::None);
        assert_eq!(code_out.get(&mock_b).unwrap().data.extra_field.inner.len(),0);
        assert_eq!(code_out.get(&mock_b).unwrap().data.extra.inner.len(),0);
        
        println!("out {:?}\n\n",code_out.get(&mock_b).unwrap());
        
        assert!(code_out.get(&mock_c).is_some());
        assert_eq!(code_out.get(&mock_c).unwrap().get_name(),mock_c);
        assert_eq!(code_out.get(&mock_c).unwrap().ty,ModelType::All);
        assert_eq!(code_out.get(&mock_c).unwrap().data.extra_field.inner.len(),2);
        assert_eq!(code_out.get(&mock_c).unwrap().data.extra.inner.len(),2);

        println!("out {:?}\n\n",code_out.get(&mock_c).unwrap());
    }
}
