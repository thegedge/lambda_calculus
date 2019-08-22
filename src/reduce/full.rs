//! Full beta reduction strategy
use crate::parser::Term;
use crate::substitution::Substitutable;

use super::Reduction;

/// Tag struct to be used as the `Strategy` in `Reducible`
pub struct Full;

impl Full {
    pub fn new() -> Full {
        Full {}
    }
}

impl Reduction for Full {
    type Term = Term;

    fn reduce(&self, term: &Self::Term) -> Self::Term {
        match term {
            Term::Variable(_) => {
                term.clone()
            },
            Term::Abstraction(s, t) => {
                Term::Abstraction(s.clone(), box self.reduce(t))
            },
            Term::Application(t1, t2) => {
                let t1_reduced = self.reduce(t1);
                let t2_reduced = self.reduce(t2);
                match t1_reduced {
                    Term::Abstraction(name, body) => {
                        self.reduce(&body.substitute(name, &t2_reduced))
                    },
                    _ => {
                        Term::Application(box t1_reduced, box t2_reduced)
                    }
                }
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::{l, a, v};
    use super::*;

    fn assert_reduces_to(expected: Term, expr: Term) {
        assert_eq!(
            expected,
            Full::new().reduce(&expr)
        )
    }

    #[test]
    pub fn test_does_not_reduce_variable() {
        assert_reduces_to(
            v("x"),
            v("x")
        );
    }

    #[test]
    pub fn test_does_not_reduce_simple_abstraction() {
        assert_reduces_to(
            l("x", "x"),
            l("x", "x")
        );
    }

    #[test]
    pub fn test_does_not_reduce_simple_application() {
        assert_reduces_to(
            a("x", l("y", "y")),
            a("x", l("y", "y"))
        );
    }

    #[test]
    pub fn test_reduces_inside_abstraction() {
        assert_reduces_to(
            l("x", a("z", "x")),
            l("x", a(a(l("y", "y"), "z"), "x"))
        );
    }

    #[test]
    pub fn test_reduces_simple_application() {
        assert_reduces_to(
            v("x"),
            a(l("y", "y"), "x")
        );
    }

    #[test]
    pub fn test_reduces_complex_application() {
        assert_reduces_to(
            l("z", "z"),
            a(l("y", "y"), l("z", "z"))
        );
    }

    #[test]
    pub fn test_reduce_with_overlapping_variables() {
        assert_reduces_to(
            l("z", "z"),
            a(l("y", "y"), l("z", "z"))
        );
    }
}
