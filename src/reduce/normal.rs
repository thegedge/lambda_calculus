//! Normal order reduction strategy
use crate::parser::Term;
use crate::substitution::Substitutable;

use super::Reduction;

pub struct Normal;

impl Normal {
    pub fn new() -> Normal {
        Normal {}
    }
}

impl Reduction for Normal {
    type Term = Term;

    fn reduce(&self, term: &Self::Term) -> Self::Term {
        match term {
            Term::Variable(_) => {
                term.clone()
            },
            Term::Abstraction(s, t) => {
                Term::Abstraction(s.clone(), box self.reduce(t))
            },
            Term::Application(box Term::Variable(s), t2) => {
                Term::Application(box Term::Variable(s.clone()), box self.reduce(t2))
            },
            Term::Application(box Term::Abstraction(name, body), box t2) => {
                self.reduce(&body.substitute(name.as_str(), t2))
            }
            Term::Application(t1, t2) => {
                let t1_reduced = self.reduce(t1);
                Term::Application(box t1_reduced, t2.clone())
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::parse;
    use super::*;

    fn assert_reduces_to(expected: &str, expr: &str) {
        assert_eq!(
            parse(expected).unwrap(),
            Normal::new().reduce(&parse(expr).unwrap())
        )
    }

    #[test]
    pub fn test_does_not_reduce_variable() {
        assert_reduces_to("x", "x");
    }

    #[test]
    pub fn test_does_not_reduce_simple_abstraction() {
        assert_reduces_to(r"\x.x", r"\x.x");
    }

    #[test]
    pub fn test_does_not_reduce_simple_application() {
        assert_reduces_to(r"x \y.y", r"x \y.y");
    }

    #[test]
    pub fn test_reduces_inside_abstraction() {
        assert_reduces_to(r"\x.z x", r"\x.((\y.y) z) x");
    }

    #[test]
    pub fn test_reduces_simple_application() {
        assert_reduces_to(r"x", r"(\y.y) x");
    }

    #[test]
    pub fn test_reduces_application_with_lambda_argument() {
        assert_reduces_to(r"\z.z", r"(\y.y) \z.z");
    }

    #[test]
    pub fn test_reduces_application_fully1() {
        assert_reduces_to(r"z", r"(\x.x z) \z.z");
    }

    #[test]
    pub fn test_reduces_application_fully2() {
        assert_reduces_to(r"z \z.z", r"((\x.x) z) \z.z");
    }
}