///! Traits and structs for querying things about variables in an expression

use std::collections::HashSet;

use crate::parser::Term;

/// A trait for computing free variables on an expression.
pub trait Variables {
    /// Returns the free variables in a term.
    fn free_variables(&self) -> HashSet<String>;
}

impl Variables for Term {
    fn free_variables(&self) -> HashSet<String> {
        match self {
            Term::Variable(s) => {
                vec![s.clone()].into_iter().collect()
            },
            Term::Abstraction(s, t) => {
                let mut fv = t.free_variables();
                fv.remove(s);
                fv
            },
            Term::Application(t1, t2) => {
                let mut fv = t1.free_variables();
                fv.extend(t2.free_variables().into_iter());
                fv
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::{l, a, v};

    fn assert_fvs_same(expected: Vec<&str>, t: Term) {
        assert_eq!(
            expected.into_iter().map(str::to_string).collect::<HashSet<_>>(),
            t.free_variables()
        );
    }

    #[test]
    fn test_with_variable() {
        assert_fvs_same(
            vec!["x"],
            v("x")
        );
    }

    #[test]
    fn test_with_abstraction() {
        assert_fvs_same(
            vec!["y"],
            l("x", a("x", "y"))
        );
    }

    #[test]
    fn test_with_application() {
        assert_fvs_same(
            vec!["x", "y"],
            a("x", "y")
        );
    }

    #[test]
    fn test_with_nested_abstraction() {
        assert_fvs_same(
            vec!["z"],
            l("x", l("y", a("x", a("y", "z"))))
        );
    }
}
