//! Call by value reduction strategy
//!
//! This strategy evaluates the outermost terms, but only after the right-hand side has been evaluated
//! to a value (abstractions in a simple calculus).
use crate::notation::named::Term;
use crate::substitution::Substitutable;

use super::Evaluable;

pub struct CallByValue;

impl CallByValue {
    pub fn new() -> CallByValue {
        CallByValue {}
    }
}

impl Evaluable for CallByValue {
    type Term = Term;
    type Context = super::EmptyContext;

    fn step(&self, ctx: &mut Self::Context, term: Self::Term) -> Option<Self::Term> {
        match term {
            Term::Application(box Term::Abstraction(name, body), box arg) if arg.is_value() => {
                Some(body.substitute(name.as_str(), &arg))
            },
            Term::Application(t1, box t2) if t1.is_value() => {
                self.step(ctx, t2)
                    .map(|t2_evaluated| Term::Application(t1, box t2_evaluated))
            },
            Term::Application(box t1, t2) => {
                self.step(ctx, t1)
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
    use crate::evaluation::EmptyContext;
    use super::*;

    fn assert_evaluates_to(expected: &str, expr: &str) {
        assert_eq!(
            parse_one(expected).unwrap(),
            CallByValue::new().evaluate(&mut EmptyContext{}, parse_one(expr).unwrap())
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
    pub fn test_does_not_evaluate_application_with_variable_on_left() {
        assert_evaluates_to(r"x \y.y", r"x \y.y");
    }

    #[test]
    pub fn test_does_not_evaluate_inside_abstraction() {
        assert_evaluates_to(r"\x.(\y.y) z", r"\x.(\y.y) z");
    }

    #[test]
    pub fn test_evaluates_simple_application() {
        assert_evaluates_to(r"\z.z", r"(\y.y) (\z.z)");
    }

    #[test]
    pub fn test_does_not_evaluate_application_without_a_value_argument() {
        assert_evaluates_to(r"(\y.y) x", r"(\y.y) x");
    }

    #[test]
    pub fn test_evaluates_application_with_lambda_argument() {
        assert_evaluates_to(r"\z.z", r"(\y.y) \z.z");
    }

    #[test]
    pub fn test_evaluates_two_argument_application() {
        assert_evaluates_to(r"\x.x", r"(\t.\f.t) (\x.x) (\y.y)");
    }

    #[test]
    pub fn test_evaluates_three_argument_application() {
        assert_evaluates_to(r"\z.z", r"(\x.\y.\z. x y z) (\x.x) (\y.y) (\z.z)");
    }

    #[test]
    pub fn test_evaluates_with_and_tru_fls() {
        assert_evaluates_to(r"\t.\f.f", r"(\a.\b.(a b) (\t.\f.f)) (\t.\f.t) (\t.\f.f)");
    }
}
