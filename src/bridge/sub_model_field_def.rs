use syn::{Ident, Path, Type};

use crate::darling_models::{
    field_input::{FieldItem, HaveField, TypeMapping},
    struct_input::{ExtraField, ModelFieldCaptureType},
    utils::{ExtraAttrs, Vis},
};
#[derive(Debug)]
pub struct SubModelFieldDef {
    pub vis: Vis,
    pub extra: ExtraAttrs,
    pub define: SubModelField,
}
#[derive(Debug)]
pub enum SubModelField {
    FromRoot(SubModelFieldFromRoot),
    Extra(SubModelFieldExtra),
}
#[derive(Debug)]
/// field mapping from root model
pub struct SubModelFieldFromRoot {
    pub root_name: Ident,
    pub root_type: Type,
    pub field_type_mapping: FieldFromRootTypeMapping,
    pub filed_ident_mapping: Option<Ident>,
}

impl SubModelFieldFromRoot {
    fn new_with_empty(root_name: &Ident, root_ty: &Type) -> Self {
        SubModelFieldFromRoot {
            root_name: root_name.to_owned(),
            root_type: root_ty.to_owned(),
            field_type_mapping: FieldFromRootTypeMapping::Direct,
            filed_ident_mapping: None,
        }
    }
}

impl SubModelFieldFromRoot {
    fn from_have_field(
        root_name: &Ident,
        root_ty: &Type,
        to_type: Option<TypeMapping>,
        to_name: Option<Ident>,
    ) -> Self {
        let field_type_mapping = FieldFromRootTypeMapping::from_option_type_mapping(to_type);

        let from_model = SubModelFieldFromRoot {
            root_name: root_name.to_owned(),
            root_type: root_ty.to_owned(),
            field_type_mapping,
            filed_ident_mapping: to_name,
        };
        from_model
    }
}
#[derive(Debug)]
/// field only in sub model
pub struct SubModelFieldExtra {
    pub ty: Type,
    pub name: Ident,
    pub constructor: Path,
}
#[derive(Debug)]
pub enum FieldFromRootTypeMapping {
    /// direct comes from root and type
    Direct,
    /// mapping from root type to target type
    MappingType { map_ty: Type, mapper: Path },
}

impl FieldFromRootTypeMapping {
    fn from_option_type_mapping(ty_map: Option<TypeMapping>) -> Self {
        match ty_map {
            Some(TypeMapping {
                target_type,
                mapping_fun,
            }) => FieldFromRootTypeMapping::MappingType {
                map_ty: target_type,
                mapper: mapping_fun,
            },
            None => FieldFromRootTypeMapping::Direct,
        }
    }
}

impl SubModelFieldDef {
    pub fn from_field_item(
        field_item: &mut FieldItem,
        this_sub_model: &Ident,
        sub_model_capture_type: &ModelFieldCaptureType,
    ) -> syn::Result<Option<Self>> {
        use crate::darling_models::field_input;
        let v = if let Some(sfd) = field_item.sub_model_field_defines.remove(this_sub_model) {
            match (sfd, sub_model_capture_type) {
                (field_input::SubModelFieldDef::Ignore(_), ModelFieldCaptureType::All) => None,
                (
                    field_input::SubModelFieldDef::Have(
                        field_input::HaveStatus::Having,
                        HaveField {
                            vis,
                            to_name,
                            to_type,
                            extra,
                            ..
                        },
                    ),
                    ModelFieldCaptureType::All,
                )
                | (
                    field_input::SubModelFieldDef::Have(
                        field_input::HaveStatus::Want,
                        HaveField {
                            vis,
                            to_name,
                            to_type,
                            extra,
                            ..
                        },
                    ),
                    ModelFieldCaptureType::None,
                ) => {
                    let from_model = SubModelFieldFromRoot::from_have_field(
                        &field_item.name,
                        &field_item.ty,
                        to_type,
                        to_name,
                    );

                    Some(SubModelFieldDef {
                        vis,
                        define: SubModelField::FromRoot(from_model),
                        extra,
                    })
                }

                (
                    field_input::SubModelFieldDef::Have(field_input::HaveStatus::Want, w),
                    ModelFieldCaptureType::All,
                ) => {
                    return Err(syn::Error::new(
                        w.owner.span(),
                        "`all` Capture type cannot using `want`, you can using `having`",
                    ))
                }
                (
                    field_input::SubModelFieldDef::Have(field_input::HaveStatus::Having, i),
                    ModelFieldCaptureType::None,
                ) => {
                    return Err(syn::Error::new(
                        i.owner.span(),
                        "`none` capture type cannot using `having`",
                    ))
                }
                (field_input::SubModelFieldDef::Ignore(i), ModelFieldCaptureType::None) => {
                    return Err(syn::Error::new(
                        i.owner.span(),
                        "`none` Capture type cannot using `ignore`",
                    ))
                }
            }
        } else {
            match sub_model_capture_type {
                ModelFieldCaptureType::All => Some(Self {
                    vis: Vis::default(),
                    extra: ExtraAttrs::default(),
                    define: SubModelField::FromRoot(SubModelFieldFromRoot::new_with_empty(
                        &field_item.name,
                        &field_item.ty,
                    )),
                }),
                ModelFieldCaptureType::None => None,
            }
        };

        Ok(v)
    }

    pub fn from_extra_field(extra_field: ExtraField) -> syn::Result<Self> {
        let ExtraField { name, extra } = extra_field;
        let v = Self {
            vis: extra.vis,
            extra: extra.extra,
            define: SubModelField::Extra(SubModelFieldExtra {
                ty: extra.ty,
                name,
                constructor: extra.create,
            }),
        };
        Ok(v)
    }
}
