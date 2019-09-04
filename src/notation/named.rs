use std::{
    fmt,
};

use super::Notation;

#[derive(Clone, PartialEq)]
pub enum Term {
    Variable(String),
    Abstraction(String, Box<Term>),
    Application(Box<Term>, Box<Term>),
}

impl Term {
    pub fn is_redex(&self) -> bool {
        match self {
            Term::Application(box Term::Abstraction(_, _), _) => true,
            Term::Application(t1, t2) => t1.is_redex() || t2.is_redex(),
            _ => false,
        }
    }

    pub fn is_value(&self) -> bool {
        match self {
            Term::Abstraction(_, _) => true,
            _ => false,
        }
    }
}

impl Notation for Term {
    type VariableName = String;

    /// Returns a term representing the application of `arg` to `func`
    fn application(func: Self, arg: Self) -> Self {
        Term::Application(box func, box arg)
    }

    /// Returns an abstraction term with the given body and bound variable name
    fn abstraction(bound_var_name: Self::VariableName, body: Self) -> Self {
        Term::Abstraction(bound_var_name, box body)
    }

    /// Returns a variable term with the given name
    fn variable(name: Self::VariableName) -> Self {
        Term::Variable(name)
    }
}

impl fmt::Display for Term {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Term::Variable(s) => write!(f, "{}", s),
            Term::Abstraction(s, t) => write!(f, "λ{}.{}", s, t),
            Term::Application(box Term::Variable(t1), box Term::Variable(t2)) => write!(f, "{} {}", t1, t2),
            Term::Application(box Term::Variable(t1), t2) => write!(f, "{} ({})", t1, t2),
            Term::Application(t1, box Term::Variable(t2)) => write!(f, "({}) {}", t1, t2),
            Term::Application(t1, t2) => write!(f, "({}) ({})", t1, t2),
        }
    }
}

impl fmt::Debug for Term {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Term::Variable(s) => write!(f, "{:?}", s),
            Term::Abstraction(s, t) => write!(f, "l({:?}, {:?})", s, t),
            Term::Application(t1, t2) => write!(f, "a({:?}, {:?})", t1, t2),
        }
    }
}

pub fn l<S, T>(name: S, body: T) -> Term
    where S: Into<String>,
          T: Into<Term>
{
    Term::Abstraction(name.into(), Box::new(body.into()))
}

pub fn a<T1, T2>(a: T1, b: T2) -> Term
    where T1: Into<Term>,
          T2: Into<Term>
{
    Term::Application(Box::new(a.into()), Box::new(b.into()))
}

pub fn v<T: Into<String>>(name: T) -> Term {
    Term::Variable(name.into())
}

impl <T> From<T> for Term
    where T: Into<String>
{
    fn from(s: T) -> Term {
        Term::Variable(s.into())
    }
}

