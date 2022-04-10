mod only_nest_meta_list;
mod err_prefab;
mod vis;
mod extra_attr;
mod from_meta_list;
mod nest_meta_list;

pub use extra_attr::ExtraAttrs;
pub use from_meta_list::load_from_meta_list;
pub use nest_meta_list::MetaList;
pub use vis::Vis;
pub use err_prefab::darling_unknown_format;
pub use err_prefab::darling_duplicate_field;
pub use err_prefab::darling_custom;
pub use only_nest_meta_list::only_neat_meta_list;

pub const ATTR_NAME: &str = "sub_model";


#[cfg(test)]
macro_rules! code {
    ($code:literal=>$t:ty) => {
        {
            let code = $code;
            syn::parse_str::<$t>(code).expect("Bad Code")   
        }
    };
    ($t:ty : $code:literal)=>{
        {
            let code = $code;
            syn::parse_str::<$t>(code).expect("Bad Code")   
        }
    }
}