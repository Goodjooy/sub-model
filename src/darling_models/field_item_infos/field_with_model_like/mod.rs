use crate::{
    bridges::{LikeLoader, LikeTo, LoadingModelInfo},
    darling_models::utils::FieldServer,
};

use super::FieldModelInput;

pub struct FieldWithLike<'m, T, S>
where
    T: LoadingModelInfo + LikeLoader,
{
    model: &'m T,
    server: S,
}

impl<'m, T, S> FieldWithLike<'m, T, S>
where
    T: LoadingModelInfo + LikeLoader,
{
    pub fn new(model: &'m T, server: S) -> Self {
        Self { model, server }
    }
}

impl<'m, T, S, In> FieldServer<In> for FieldWithLike<'m, T, S>
where
    T: LoadingModelInfo + LikeLoader,
    S: FieldServer<In, Output = FieldModelInput>,
{
    type Output = S::Output;

    type Error = S::Error;

    fn proc(&self, input: In) -> Result<Self::Output, Self::Error> {
        let mut input = self.server.proc(input)?;

        for (model, like) in self
            .model
            .all_models()
            .map(|id| self.model.model_like_to(id).map(|l| (id, l)))
            .filter_map(|v| v)
            .map(|(id, like)| {
                input
                    .model_having
                    .get_mut(id)
                    .map(|mut_mod| (mut_mod, like))
            })
            .filter_map(|v| v)
        {
            
        }
        unimplemented!()
    }
}
