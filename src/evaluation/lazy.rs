//! Lazy (call by name) reduction strategy
//!
//! This strategy uses the normal order strategy, but does not evaluate within abstractions.
use crate::notation::named::Term;
use crate::substitution::Substitutable;

use super::Evaluable;

pub struct Lazy;

impl Lazy {
    pub fn new() -> Lazy {
        Lazy {}
    }
}

impl Evaluable for Lazy {
    type Term = Term;

    fn step(&self, term: Self::Term) -> Option<Self::Term> {
        match term {
            Term::Application(box Term::Abstraction(name, body), box arg) => {
                Some(body.substitute(name.as_str(), &arg))
            },
            Term::Application(box t1, t2) if t1.is_redex() => {
                self.step(t1)
                    .map(|t1_evaluated| Term::Application(box t1_evaluated, t2))
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

    fn assert_evaluates_to(expected: &str, expr: &str) {
        assert_eq!(
            parse_one(expected).unwrap(),
            Lazy::new().evaluate(parse_one(expr).unwrap())
        )
    }

    #[test]
    pub fn test_does_not_evaluate_variable() {
        assert_evaluates_to("x", "x");
    }

    #[test]
    pub fn test_does_not_evaluate_simple_abstraction() {
        assert_evaluates_to(r"\x.x", r"\x.x");
    }

    #[test]
    pub fn test_does_not_evaluate_simple_application() {
        assert_evaluates_to(r"x \y.y", r"x \y.y");
    }

    #[test]
    pub fn test_does_not_evaluate_inside_abstraction() {
        assert_evaluates_to(r"\x.(\y.y) z", r"\x.(\y.y) z");
    }

    #[test]
    pub fn test_evaluates_simple_application() {
        assert_evaluates_to(r"x", r"(\y.y) x");
    }

    #[test]
    pub fn test_evaluates_application_with_lambda_argument() {
        assert_evaluates_to(r"\z.z", r"(\y.y) \z.z");
    }

    #[test]
    pub fn test_does_not_evaluate_application_with_variable_on_left() {
        assert_evaluates_to(r"z (\x.x) (\z.z)", r"z (\x.x) (\z.z)");
    }

    #[test]
    pub fn test_evaluates_application_fully1() {
        assert_evaluates_to(r"z", r"(\x.x z) \z.z");
    }

    #[test]
    pub fn test_evaluates_application_fully2() {
        assert_evaluates_to(r"z \z.z", r"((\x.x) z) \z.z");
    }
}
