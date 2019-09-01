///! Traits and structs for querying things about variables in an expression

use std::collections::HashSet;

use crate::notation::named::Term;

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
    use crate::parser::parse_one;

    fn assert_fvs_same(expected: Vec<&str>, t: &str) {
        assert_eq!(
            expected.into_iter().map(str::to_string).collect::<HashSet<_>>(),
            parse_one(t).unwrap().free_variables()
        );
    }

    #[test]
    fn test_with_variable() {
        assert_fvs_same(
            vec!["x"],
            "x"
        );
    }

    #[test]
    fn test_with_abstraction() {
        assert_fvs_same(
            vec!["y"],
            r"\x.x y"
        );
    }

    #[test]
    fn test_with_application() {
        assert_fvs_same(
            vec!["x", "y"],
            "x y"
        );
    }

    #[test]
    fn test_with_nested_abstraction() {
        assert_fvs_same(
            vec!["z"],
            r"\x.\y.x y z"
        );
    }

    #[test]
    fn test_with_nested_abstraction_having_overlapping_bound_names() {
        assert_fvs_same(
            vec![],
            r"\f.\t.\f.f"
        );
    }
}
