use darling::FromAttributes;
use syn::{Meta, NestedMeta};

/// loading meta list from attrs
pub struct MetaList(Vec<Meta>);

impl MetaList {
    pub fn filter_with_ident(self, ident: &'static str) -> Self {
        self.into_iter()
            .filter(|meta| meta.path().is_ident(ident))
            .collect()
    }

    pub fn false_filter_with_ident(self, ident: &'static str) -> Self {
        self.into_iter()
            .filter(|meta| !meta.path().is_ident(ident))
            .collect()
    }

    pub fn group_into_nest_meta(self) -> Vec<NestedMeta> {
        self.into_iter()
            .filter_map(|meta| match meta {
                Meta::List(meta_inner) => Some(meta_inner.nested),
                _ => None,
            })
            .map(|p| p.into_iter().collect::<Vec<_>>())
            .reduce(|mut l, r| {
                l.extend(r);
                l
            })
            .unwrap_or_default()
    }
}

impl IntoIterator for MetaList {
    type IntoIter = std::vec::IntoIter<Meta>;
    type Item = Meta;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl FromIterator<Meta> for MetaList {
    fn from_iter<T: IntoIterator<Item = Meta>>(iter: T) -> Self {
        Self(Vec::from_iter(iter))
    }
}

impl FromAttributes for MetaList {
    fn from_attributes(attrs: &[syn::Attribute]) -> darling::Result<Self> {
        let mut meta_list = Vec::with_capacity(attrs.len());

        for attr in attrs {
            meta_list.push(attr.parse_meta()?);
        }

        Ok(Self(meta_list))
    }
}
