use super::Term as Base;

pub struct Term<M>(M, Base<u32>);

impl <M> AsRef<Base<u32>> for Term<M> {
    fn as_ref(&self) -> &Base<u32> {
        &self.1
    }
}
