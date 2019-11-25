use super::Term as Base;

pub struct Term<C>(C, Base<String>);

impl <C> AsRef<Base<String>> for Term<C> {
    fn as_ref(&self) -> &Base<String> {
        &self.1
    }
}
