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

    fn step(&self, term: Self::Term) -> Option<Self::Term> {
        match term {
            Term::Application(box Term::Abstraction(name, body), box arg) if arg.is_value() => {
                Some(body.substitute(name.as_str(), &arg))
            },
            Term::Application(t1, box t2) if t1.is_value() => {
                self.step(t2)
                    .map(|t2_reduced| Term::Application(t1, box t2_reduced))
            },
            Term::Application(box t1, t2) => {
                self.step(t1)
                    .map(|t1_reduced| Term::Application(box t1_reduced, t2))
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
            CallByValue::new().reduce(parse_one(expr).unwrap())
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
        assert_reduces_to(r"\z.z", r"(\y.y) (\z.z)");
    }

    #[test]
    pub fn test_does_not_reduce_application_without_a_value_argument() {
        assert_reduces_to(r"(\y.y) x", r"(\y.y) x");
    }

    #[test]
    pub fn test_reduces_application_with_lambda_argument() {
        assert_reduces_to(r"\z.z", r"(\y.y) \z.z");
    }

    #[test]
    pub fn test_reduces_two_argument_application() {
        assert_reduces_to(r"\x.x", r"(\t.\f.t) (\x.x) (\y.y)");
    }

    #[test]
    pub fn test_reduces_three_argument_application() {
        assert_reduces_to(r"\z.z", r"(\x.\y.\z. x y z) (\x.x) (\y.y) (\z.z)");
    }

    #[test]
    pub fn test_reduces_with_and_tru_fls() {
        assert_reduces_to(r"\t.\f.f", r"(\a.\b.(a b) (\t.\f.f)) (\t.\f.t) (\t.\f.f)");
    }
}
