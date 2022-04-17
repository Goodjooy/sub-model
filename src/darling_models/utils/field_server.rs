/// sync server like `tower::Server`
pub trait FieldServer<Input> {
    type Output;
    type Error: Into<darling::Error>;

    fn proc(&self, input: Input) -> Result<Self::Output, Self::Error>;
}
