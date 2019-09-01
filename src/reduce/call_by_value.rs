//! Call by value reduction strategy
//!
//! This strategy reduces the outermost terms, but only after the right-hand side has been reduced
//! to a value (abstractions in a simple calculus).
use crate::notation::named::Term;
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
            Term::Application(t1, t2) if t1.is_redex() => {
                let t1_reduced = self.reduce(t1);
                self.reduce(&Term::Application(box t1_reduced, t2.clone()))
            },
            Term::Application(t1, t2) if t2.is_redex() => {
                let t2_reduced = self.reduce(t2);
                self.reduce(&Term::Application(t1.clone(), box t2_reduced))
            },
            Term::Application(box Term::Abstraction(name, body), arg) => {
                self.reduce(&body.substitute(name.as_str(), &arg))
            },
            _ => {
                term.clone()
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
            CallByValue::new().reduce(&parse_one(expr).unwrap())
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
    pub fn test_does_not_reduce_application_with_variable_on_left() {
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
    pub fn test_reduces_application_fully1() {
        assert_reduces_to(r"z", r"(\x.x z) \z.z");
    }

    #[test]
    pub fn test_reduces_application_fully2() {
        assert_reduces_to(r"z \z.z", r"((\x.x) z) \z.z");
    }

    #[test]
    pub fn test_reduces_two_argument_application() {
        assert_reduces_to(r"a", r"(\t.\f.t) a b");
    }

    #[test]
    pub fn test_reduces_three_argument_application() {
        assert_reduces_to(r"a b c", r"(\x.\y.\z. x y z) a b c");
    }

    #[test]
    pub fn test_reduces_with_and_tru_fls() {
        assert_reduces_to(r"\t.\f.f", r"(\a.\b.(a b) (\t.\f.f)) (\t.\f.t) (\t.\f.f)");
    }
}
