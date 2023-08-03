#[derive(Debug, Clone, darling::FromMeta)]
/// mapping from src type to target type
pub struct TypeMapping {
    /// the target type want to mapping
    #[darling(rename = "ty")]
    pub target_type: syn::Type,
    /// the mapping function,
    /// its format is  fn (src:ty)->ty
    #[darling(rename = "by")]
    pub mapping_fun: syn::Path,
}
