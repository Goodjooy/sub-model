use std::collections::HashMap;

use syn::{Ident, Type};

use crate::darling_models::{
    field_item_infos::{FieldItem, FieldType, HaveField, HaveStatus, TypeMapping},
    struct_item_infos::{ExtraField, ExtraFields, ModelType},
    utils::darling_duplicate_field,
    FromIdent,
};

/// the field comes from Parent Model
#[derive(Debug, Clone)]
pub struct SrcField {
    /// the field name in Parent Model
    pub src_name: Ident,
    /// the field type in Parent Model
    pub src_ty: Type,
    /// the field define of special SubModel
    pub extra_info: HaveField,
}

impl SrcField {
    pub fn get_tgt_name(&self) -> Ident {
        (self.extra_info.to_name.clone()).unwrap_or(self.src_name.clone())
    }

    pub fn get_tgt_type(&self) -> Type {
        let tm: Option<&TypeMapping> = (&self.extra_info.to_type).into();

        tm.map(|m| m.target_type.clone())
            .unwrap_or(self.src_ty.clone())
    }
}

/// each field in SubModel
#[derive(Debug, Clone)]
pub enum ModelField {
    /// the Field Come from Parent Model
    Src(SrcField),
    /// the field is extra
    Extra(ExtraField),
}

impl ModelField {
    pub fn get_field_name(&self) -> Ident {
        match self {
            ModelField::Src(src) => src.get_tgt_name(),
            ModelField::Extra(ext) => ext.name.clone(),
        }
    }
}

#[derive(Debug)]
pub struct SubModelFields {
    pub inner: HashMap<Ident, ModelField>,
}

impl SubModelFields {
    pub fn from_fields(
        owner: &Ident,
        ty: &ModelType,
        field_items: &impl AsRef<[FieldItem]>,
    ) -> darling::Result<Self> {
        let fields = field_items
            .as_ref()
            .into_iter()
            .filter_map(|item| {
                Some(match (item.sub_models.get(owner), ty) {
                    (None, ModelType::All) => ModelField::Src(SrcField {
                        src_name: item.name.clone(),
                        src_ty: item.ty.clone(),
                        extra_info: FromIdent::form_ident(owner.clone()),
                    }),
                    (Some(FieldType::Ignore(_)), ModelType::All) => None?,
                    (Some(FieldType::Have(HaveStatus::Having, have)), ModelType::All) => {
                        ModelField::Src(SrcField {
                            src_name: item.name.clone(),
                            src_ty: item.ty.clone(),
                            extra_info: have.clone(),
                        })
                    }
                    (Some(FieldType::Have(HaveStatus::Want, have)), ModelType::None) => {
                        ModelField::Src(SrcField {
                            src_name: item.name.clone(),
                            src_ty: item.ty.clone(),
                            extra_info: have.clone(),
                        })
                    }
                    (None, ModelType::None) => None?,
                    (Some(FieldType::Have(HaveStatus::Want, _)), ModelType::All) => {
                        panic!("All Type SubModel Using Want")
                    }
                    (Some(FieldType::Have(HaveStatus::Having, _)), ModelType::None) => {
                        panic!("None Type SubModel Using Having")
                    }
                    (Some(FieldType::Ignore(_)), ModelType::None) => {
                        panic!("None Type SubModel Using Ignore")
                    }
                })
            })
            .collect::<Vec<_>>();
        let mut inner = HashMap::with_capacity(fields.len());
        for field in fields {
            let name = field.get_field_name();
            if let Some(_) = inner.insert(name.clone(), field) {
                darling_duplicate_field(&name)?;
            }
        }

        Ok(Self { inner })
    }

    pub fn adding_extras(mut self, extra_fields: &ExtraFields) -> darling::Result<Self> {
        for (name, extra) in &extra_fields.inner {
            if let Some(_) = self
                .inner
                .insert(name.clone(), ModelField::Extra(extra.clone()))
            {
                darling_duplicate_field(name)?;
            }
        }
        Ok(self)
    }
}
