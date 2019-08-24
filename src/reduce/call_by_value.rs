//! Call by value reduction strategy
//!
//! This strategy reduces the outermost terms, but only after the right-hand side has been reduced
//! to a value (abstractions in a simple calculus).
use crate::parser::Term;
use crate::substitution::Substitutable;

use super::Reduction;

pub struct CallByValue;

impl CallByValue {
    pub fn new() -> CallByValue {
        CallByValue {}
    }
}

impl Reduction for CallByValue {
    type Term = Term;

    fn reduce(&self, term: &Self::Term) -> Self::Term {
        match term {
            Term::Application(box Term::Abstraction(name, body), arg) => {
                let arg_reduced = self.reduce(arg);
                self.reduce(&body.substitute(name.as_str(), &arg_reduced))
            },
            Term::Application(t1, t2) => {
                let t1_reduced = self.reduce(t1);

                // Check if we have a redex
                match t1_reduced {
                    Term::Abstraction(_, _) => self.reduce(&Term::Application(box t1_reduced, t2.clone())),
                    _ => Term::Application(box t1_reduced, t2.clone()),
                }
            },
            _ => {
                term.clone()
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
            CallByValue::new().reduce(&parse(expr).unwrap())
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
    pub fn test_does_not_reduce_inside_abstraction() {
        assert_reduces_to(r"\x.(\y.y) z", r"\x.(\y.y) z");
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

    #[test]
    pub fn test_reduces_multi_argument_application() {
        assert_reduces_to(r"a", r"(\t.\f.t) a b");
    }
}
