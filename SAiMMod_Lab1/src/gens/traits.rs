
pub trait Generate {
    type Output;

    fn generate(&mut self) -> Option<Self::Output>;
}

pub trait IntoGenerator {
    type Output;
    type IntoGen: Generate<Output=Self::Output>;

    fn into_gen(self, count: u32) -> Self::IntoGen;
}

