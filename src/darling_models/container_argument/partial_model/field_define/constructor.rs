use darling::FromMeta;
use syn::spanned::Spanned;
use syn::{ExprClosure, Path};

#[derive(Debug)]
pub enum Constructor {
    Path(Path),
    Closure(ExprClosure),
}

impl FromMeta for Constructor {
    fn from_string(value: &str) -> darling::Result<Self> {
        syn::parse_str::<Path>(value)
            .map(Constructor::Path)
            .or_else(|_| syn::parse_str::<ExprClosure>(value).map(Constructor::Closure))
            .map_err(darling::error::Error::from)
            .map_err(|e| e.with_span(&value.span()))
    }
}

#[cfg(test)]
mod test {
    use darling::FromMeta;
    use syn::Meta;

    use crate::darling_models::container_argument::partial_model::field_define::constructor::Constructor;

    #[test]
    fn test_constructor_closure() {
        let ast =
            code!(Meta : r#"constructor = "|owner:bool, foo:&mut i32, bar:String|parent.foo""#);

        let v = Constructor::from_meta(&ast).expect("Bad Parse");
        assert!(matches!(v, Constructor::Closure(_)));
    }

    #[test]
    fn test_constructor_path() {
        let ast = code!(Meta : r#"constructor = "From::from""#);

        let v = Constructor::from_meta(&ast).expect("Bad Parse");

        assert!(matches!(v, Constructor::Path(_)));
    }
}
