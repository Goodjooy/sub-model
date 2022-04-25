use syn::{Ident, Path};

#[derive(Debug, Clone)]
pub enum FieldValueFrom {
    FromSuper {
        from: Ident,
        by: Option<Path>,
        to: Option<Ident>,
    },
    FromNil {
        to: Ident,
        by: Path,
    },
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

    pub fn to_token(self, parent: &Ident) -> proc_macro2::TokenStream {
        let token = match self {
            FieldValueFrom::FromSuper {
                ref from,
                by,
                ref to,
            } => {
                let v = by
                    .map(|path| quote::quote! {#path(#parent.#from)})
                    .unwrap_or(quote::quote! {#parent.#from});

                let k = Into::<Option<&Ident>>::into(to).unwrap_or(from);
                quote::quote! {#k : #v}
            }
            FieldValueFrom::FromNil { to, by } => quote::quote! {
                #to : #by ()
            },
        };

        token
    }
}
