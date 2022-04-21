mod field_def;
mod field_model_type;
use std::collections::HashMap;

use syn::NestedMeta;

use crate::{
    bridges::{LoadingModelInfo, ModelType},
    darling_models::utils::{darling_duplicate_field, FieldServer},
};

use super::field_types_init::{self, FieldInput, FieldItem, HaveStatus};

pub use field_def::FieldDef;
pub use field_model_type::FieldModelInput;

pub struct FieldWithModelTypeServer<'s, M: LoadingModelInfo> {
    model: &'s M,
}

impl<'s, M: LoadingModelInfo> FieldWithModelTypeServer<'s, M> {
    pub fn new(model_infos: &'s M) -> Self {
        Self { model: model_infos }
    }
}

impl<'s, M: LoadingModelInfo> FieldServer<field_types_init::FieldItem>
    for FieldWithModelTypeServer<'s, M>
{
    type Output = FieldModelInput;

    type Error = darling::Error;

    fn proc(&self, input: field_types_init::FieldItem) -> Result<Self::Output, Self::Error> {
        let FieldItem {
            vis,
            attrs,
            name,
            ty,
            sub_models,
        } = input;

        let attrs = attrs.into_iter().map(|v| NestedMeta::Meta(v)).collect();

        let mut model_having = HashMap::new();

        for key in self.model.all_models() {
            // check where is control by head
            self.model
                .head_ctrl(key)
                .ok_or(darling::Error::unknown_value(&key.to_string()))
                .and_then(|b| {
                    if b {
                        Err(darling::Error::unsupported_format(
                            "Want/Ignore set by head",
                        ))
                    } else {
                        Ok(())
                    }
                })?;
                
            match self.model.model_type(key).unwrap() {
                ModelType::All => {
                    if let Some(input) = sub_models.remove(key) {
                        match input {
                            // have , load from have
                            FieldInput::Have(HaveStatus::Having, have) => {
                                // the model is exist
                                if let Some(_) = model_having.insert(key.clone(), have.into()) {
                                    darling_duplicate_field(key)?;
                                }
                            }
                            // using want error
                            FieldInput::Have(HaveStatus::Want, _) => Err(
                                darling::Error::unsupported_format("`all` Type Using `want`"),
                            )?,
                            // ignore not set
                            FieldInput::Ignore(_) => {}
                        }
                    } else {
                        if let Some(_) = model_having.insert(key.clone(), Default::default()) {
                            darling_duplicate_field(key)?;
                        }
                    }
                }
                ModelType::None => {
                    if let Some(input) = sub_models.remove(key) {
                        match input {
                            FieldInput::Ignore(_) => Err(darling::Error::unsupported_format(
                                "`none` type Using `ignore`",
                            ))?,
                            FieldInput::Have(HaveStatus::Having, _) => Err(
                                darling::Error::unsupported_format("`none` type Using `having`"),
                            )?,
                            FieldInput::Have(HaveStatus::Want, have) => {
                                if let Some(_) = model_having.insert(key.clone(), have.into()) {
                                    darling_duplicate_field(key)?;
                                }
                            }
                        }
                    }
                }
            }
        }

        Ok(FieldModelInput {
            attrs,
            vis,
            name,
            ty,
            model_having,
        })
    }
}

#[cfg(test)]
mod test_mapping {}
