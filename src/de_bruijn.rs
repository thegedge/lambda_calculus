use std::{
    fmt,
};

pub struct Term(super::Term);

pub fn l<T>(body: T) -> Term
    where T: Into<Term>
{
    Term::Abstraction(Box::new(body.into()))
}

pub fn a<T1, T2>(a: T1, b: T2) -> Term
    where T1: Into<Term>,
          T2: Into<Term>
{
    Term::Application(Box::new(a.into()), Box::new(b.into()))
}

pub fn v<T: Into<u32>>(name: T) -> Term {
    Term::Variable(name.into())
}

impl <T> From<T> for Term
    where T: Into<u32>
{
    fn from(s: T) -> Term {
        Term::Variable(s.into())
    }
}
