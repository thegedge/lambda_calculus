//! Normal order reduction strategy
//!
//! This strategy will reduce the left and outermost terms first.
use crate::notation::named::Term;
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

    fn step(&self, term: Self::Term) -> Option<Self::Term> {
        match term {
            Term::Abstraction(arg, box body) => {
                self.step(body)
                    .map(|body_reduced| Term::Abstraction(arg, box body_reduced))
            },
            Term::Application(box Term::Abstraction(name, body), box arg) => {
                Some(body.substitute(name.as_str(), &arg))
            }
            Term::Application(box t1, t2) if t1.is_redex() => {
                self.step(t1)
                    .map(|t1_reduced| Term::Application(box t1_reduced, t2))
            },
            Term::Application(t1, box t2) => {
                self.step(t2)
                    .map(|t2_reduced| Term::Application(t1, box t2_reduced))
            },
            _ => {
                None
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::parse_one;
    use super::*;

    fn assert_reduces_to(expected: &str, expr: &str) {
        assert_eq!(
            parse_one(expected).unwrap(),
            Normal::new().reduce(parse_one(expr).unwrap())
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
    pub fn test_does_not_reduce_application_with_variable_on_left() {
        assert_reduces_to(r"z (\x.x) (\z.z)", r"z (\x.x) (\z.z)");
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
