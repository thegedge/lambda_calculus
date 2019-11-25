use std::fmt;

mod de_bruijn;
mod named;

#[derive(Clone, PartialEq)]
pub enum Term<V> {
    Variable(V),
    Abstraction(V, Box<Term<V>>),
    Application(Box<Term<V>>, Box<Term<V>>),
}

impl <V> Term<V>
    where V: Clone
{
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

impl <V> fmt::Display for Term<V>
    where V: fmt::Display
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Term::Variable(s) => write!(f, "{}", s),
            Term::Abstraction(n, t) => write!(f, "λ{}.{}", n, t),
            Term::Application(box Term::Variable(t1), box Term::Variable(t2)) => write!(f, "{} {}", t1, t2),
            Term::Application(box Term::Variable(t1), t2) => write!(f, "{} ({})", t1, t2),
            Term::Application(t1, box Term::Variable(t2)) => write!(f, "({}) {}", t1, t2),
            Term::Application(t1, t2) => write!(f, "({}) ({})", t1, t2),
        }
    }
}

impl <V> fmt::Debug for Term<V>
    where V: fmt::Debug
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Term::Variable(s) => write!(f, "{:?}", s),
            Term::Abstraction(n, t) => write!(f, "l({:?}, {:?})", n, t),
            Term::Application(t1, t2) => write!(f, "a({:?}, {:?})", t1, t2),
        }
    }
}

pub fn l<V, T>(name: V, body: T) -> Term<V>
    where V: Clone + PartialEq,
          T: Into<Term<V>>
{
    Term::Abstraction(name, Box::new(body.into()))
}

pub fn a<T1, T2, V>(a: T1, b: T2) -> Term<V>
    where V: Clone + PartialEq,
          T1: Into<Term<V>>,
          T2: Into<Term<V>>
{
    Term::Application(Box::new(a.into()), Box::new(b.into()))
}

pub fn v<V, T>(name: T) -> Term<V>
    where V: Clone + PartialEq,
          T: Into<V>
{
    Term::Variable(name.into())
}

impl <V> From<V> for Term<V>
    where V: Clone + PartialEq
{
    fn from(s: V) -> Term<V> {
        Term::Variable(s.into())
    }
}

impl <V> From<Term<V>> for Term<String>
    where V: Into<String>
{
    fn from(s: Term<V>) -> Term<String> {
        match s {
            Term::Variable(s) => Term::Variable(s.into()),
            Term::Abstraction(n, b) => Term::Abstraction(n.into(), b.into()),
            Term::Application(a, b) => Term::Application(a.into(), b.into()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_is_redex_true_for_abstraction_application() {
        assert!(
            a(l("x", "x"), "y").is_redex()
        );
    }

    #[test]
    pub fn test_is_redex_false_for_non_redex() {
        assert!(
            !a("y", l("x", "x")).is_redex()
        );
    }

    #[test]
    pub fn test_is_redex_true_for_nested_abstraction() {
        assert!(
            a(a(a(l("x", "x"), "a"), "b"), "c").is_redex()
        );
    }
}
