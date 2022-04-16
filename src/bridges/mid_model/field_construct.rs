
use syn::{Ident, Path, Type};

#[derive(Debug,Clone)]
pub enum FieldValueFrom {
    FromSuper {
        from: Ident,
        by: Option<TypeMapper>,
        to: Option<Ident>,
    },
    FromNil {
        to: Ident,
        by: Path,
    },
}

#[derive(Debug,Clone)]
pub struct TypeMapper {
    target_type: Type,
    map_function: Path,
}

impl FieldValueFrom {
    pub fn field_name(&self) -> &Ident {
        match self {
            FieldValueFrom::FromSuper { from, to: None, .. } => from,
            FieldValueFrom::FromSuper { to: Some(n), .. } => n,
            FieldValueFrom::FromNil { to, .. } => to,
        }
    }

    pub fn is_from_super(&self) -> bool {
        match self {
            FieldValueFrom::FromSuper { .. } => true,
            FieldValueFrom::FromNil { .. } => false,
        }
    }

    pub fn is_rename(&self) -> bool {
        match self {
            Self::FromSuper { to: Some(_), .. } => true,
            _ => false,
        }
    }

    pub fn is_map_type(&self) -> bool {
        match self {
            Self::FromSuper { by: Some(_), .. } => true,
            _ => false,
        }
    }

    pub fn is_extra(&self) -> bool {
        match self {
            Self::FromNil { .. } => true,
            Self::FromSuper { .. } => false,
        }
    }
}
