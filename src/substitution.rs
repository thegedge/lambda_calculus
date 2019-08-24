//! Term substitution
use std::borrow::Borrow;

use crate::parser::Term;
use crate::vars::Variables;

const PRIME: &str = "'";

/// An expression that can be substituted with another expression
pub trait Substitutable {
    /// Returns the free variables in a term.
    fn substitute<T>(&self, var: T, substitution: &Term) -> Self
        where T: Borrow<str>;
}

impl Substitutable for Term {
    fn substitute<T>(&self, var: T, substitution: &Term) -> Self
        where T: Borrow<str>
    {
        match self {
            Term::Variable(s) => {
                if s == var.borrow() {
                    substitution.clone()
                } else {
                    self.clone()
                }
            },
            Term::Abstraction(name, box t) => {
                if name == var.borrow() || t.free_variables().contains(name) {
                    println!("{} :: {} -> {} :: {:?}", self, var.borrow(), substitution, t.free_variables());
                    Term::Abstraction(
                        name.clone() + PRIME,
                        box rewrite(name, t).substitute(var.borrow(), substitution)
                    )
                } else {
                    Term::Abstraction(name.clone(), box t.substitute(var.borrow(), substitution))
                }
            },
            Term::Application(box t1, box t2) => {
                Term::Application(
                    box t1.substitute(var.borrow(), substitution),
                    box t2.substitute(var.borrow(), substitution)
                )
            },
        }
    }
}

struct Rewrite {
    name: String,
    count: usize,
}

// TODO this doesn't deal with "primed" names already being used as a bound variable name
impl Rewrite {
    fn rewrite(&mut self, term: &Term) -> Term {
        match term {
            Term::Variable(s) => {
                if &self.name == s {
                    Term::Variable(self.name.clone() + PRIME.repeat(self.count).as_str())
                } else {
                    term.clone()
                }
            },
            Term::Abstraction(name, box t) => {
                let new_name = if &self.name == name {
                    self.count += 1;
                    self.name.clone() + PRIME.repeat(self.count).as_str()
                } else {
                    name.clone()
                };

                Term::Abstraction(new_name, box self.rewrite(t))
            },
            Term::Application(box t1, box t2) => {
                Term::Application(box self.rewrite(t1), box self.rewrite(t2))
            },
        }
    }
}

fn rewrite<T: Into<String>>(name: T, term: &Term) -> Term {
    Rewrite {
        name: name.into(),
        count: 0
    }.rewrite(term)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::parse;

    fn assert_substitutes_to(expected: &str, expr: &str, subs: &str, var: &str) {
        assert_eq!(
            parse(expected).unwrap(),
            parse(expr).unwrap().substitute(var, &parse(subs).unwrap())
        )
    }

    #[test]
    fn test_variable_with_same_name() {
        assert_substitutes_to("y", "x", "y", "x");
    }

    #[test]
    fn test_variable_with_different_name() {
        assert_substitutes_to("x", "x", "z", "y");
    }

    #[test]
    fn test_application_with_overlapping_name() {
        assert_substitutes_to("x z", "x y", "z", "y");
    }

    #[test]
    fn test_application_with_none_same_name() {
        assert_substitutes_to("x y", "x y", "z", "a");
    }

    #[test]
    fn test_abstraction_with_bound_name_different() {
        assert_substitutes_to("x z", "x y", "z", "y");
    }

    #[test]
    fn test_abstraction_with_bound_name_same() {
        assert_substitutes_to(r"\x'.y y", r"\x.x y", "y", "x");
    }

    #[test]
    fn test_abstraction_with_bound_name_in_free_variables_of_substitute() {
        assert_substitutes_to(
            r"\x'.(\y.x y) y",
            r"\x.x y",
            r"\y.x y",
            "x"
        );
    }

    mod rewrite {
        use super::super::*;
        use crate::parser::parse;

        fn assert_rewrites_to(expected: &str, expr: &str, var: &str) {
            assert_eq!(
                parse(expected).unwrap(),
                rewrite(var, &parse(expr).unwrap())
            );
        }

        #[test]
        fn test_with_variable() {
            assert_rewrites_to("x", "x", "x");
        }

        #[test]
        fn test_with_application() {
            assert_rewrites_to("x y", "x y", "x");
        }

        #[test]
        fn test_simple_abstraction() {
            assert_rewrites_to(r"\x' . x' y", r"\x.x y", "x")
        }

        #[test]
        fn test_nested_abstraction_with_same_bound_variable_name() {
            assert_rewrites_to(r"\x' . x' \y . y \x''.x''", r"\x.x \y.y \x.x", "x");
        }
    }
}
