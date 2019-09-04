//! Full beta reduction strategy
//!
//! This strategy can evaluate any term at any point.
use crate::notation::named::Term;
use crate::substitution::Substitutable;

use super::Evaluable;

pub struct Full;

impl Full {
    pub fn new() -> Full {
        Full {}
    }
}

impl Evaluable for Full {
    type Term = Term;
    type Context = super::EmptyContext;

    fn step(&self, ctx: &mut Self::Context, term: Self::Term) -> Option<Self::Term> {
        match term {
            Term::Abstraction(arg, box body) => {
                self.step(ctx, body)
                    .map(|body_evaluated| Term::Abstraction(arg, box body_evaluated))
            },
            Term::Application(box t1, t2) if t1.is_redex() => {
                self.step(ctx, t1)
                    .map(|t1_evaluated| Term::Application(box t1_evaluated, t2))
            },
            Term::Application(t1, box t2) if t2.is_redex() => {
                self.step(ctx, t2)
                    .map(|t2_evaluated| Term::Application(t1, box t2_evaluated))
            },
            Term::Application(box Term::Abstraction(name, body), box arg) => {
                Some(body.substitute(name.as_str(), &arg))
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
    use crate::evaluation::EmptyContext;
    use super::*;

    fn assert_evaluates_to(expected: &str, expr: &str) {
        assert_eq!(
            parse_one(expected).unwrap(),
            Full::new().evaluate(&mut EmptyContext{}, parse_one(expr).unwrap())
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
    pub fn test_evaluates_inside_abstraction() {
        assert_evaluates_to(r"\x.z x", r"\x.((\y.y) z) x");
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
