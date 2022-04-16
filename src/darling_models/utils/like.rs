use darling::FromMeta;
use syn::Ident;

#[derive(Debug, darling::FromMeta)]
pub struct LikeInput {
    /// the target model that the `like`
    /// will to be
    ///
    /// if the model is `None` and field is `None`
    /// the model target to super model
    /// if model is `None` but field is not `None`
    /// the model target to self (like another field)
    #[darling(default)]
    model: Option<Ident>,
    /// witch filed the current field `like` to
    #[darling(default)]
    field: Option<Ident>,
}

#[derive(Debug, PartialEq, Eq,Clone)]
pub enum Like {
    LikeSuper,
    LikeSelf(Ident),
    LikeModel(Ident),
    LikeElse(Ident, Ident),
}

impl LikeInput {
    fn into_like(self) -> darling::Result<Like> {
        match (self.model, self.field) {
            (None, None) => Ok(Like::LikeSuper),
            (None, Some(f)) => Ok(Like::LikeSelf(f)),
            (Some(m), Some(f)) => Ok(Like::LikeElse(m, f)),
            (Some(m), None) => Ok(Like::LikeModel(m)),
        }
    }
}

impl FromMeta for Like {
    fn from_word() -> darling::Result<Self> {
        Ok(Like::LikeSuper)
    }

    fn from_list(items: &[syn::NestedMeta]) -> darling::Result<Self> {
        <LikeInput as FromMeta>::from_list(items).and_then(|s| s.into_like())
    }

    fn from_value(value: &syn::Lit) -> darling::Result<Self> {
        <LikeInput as FromMeta>::from_value(value)
            .and_then(|s| s.into_like())
            .map_err(|e| e.with_span(value))
    }

    fn from_string(value: &str) -> darling::Result<Self> {
        <LikeInput as FromMeta>::from_string(value)
            .and_then(|s| s.into_like())
            .map_err(|e| e.with_span(&value))
    }
}

#[cfg(test)]
mod test_like {
    use darling::FromMeta;
    use syn::NestedMeta;

    use crate::code;

    use super::Like;

    #[derive(Debug, darling::FromMeta)]
    struct Mock {
        liking: Like,
    }

    #[test]
    fn test_no_arg() {
        let item = code!(
                NestedMeta:
                r#"
                test(liking)
                "#
        );

        let out = Mock::from_nested_meta(&item).unwrap();

        assert_eq!(out.liking, Like::LikeSuper);

        println!("{:?}", out);
    }
}
