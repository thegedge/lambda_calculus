//! Term substitution
use std::borrow::Borrow;

use crate::parser::Term;
use crate::vars::Variables;

const PRIME: &str = "′";

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
    use super::PRIME;
    use super::*;
    use crate::parser::{l, a, v};

    fn prime(s: &str) -> String {
        s.to_string() + PRIME
    }

    #[test]
    fn test_variable_with_same_name() {
        assert_eq!(
            v("y"),
            v("x").substitute("x", &v("y"))
        )
    }

    #[test]
    fn test_variable_with_different_name() {
        assert_eq!(
            v("x"),
            v("x").substitute("y", &v("z"))
        )
    }

    #[test]
    fn test_application_with_overlapping_name() {
        assert_eq!(
            a("x", "z"),
            a("x", "y").substitute("y", &v("z"))
        )
    }

    #[test]
    fn test_application_with_none_same_name() {
        assert_eq!(
            a("x", "y"),
            a("x", "y").substitute("a", &v("z"))
        )
    }

    #[test]
    fn test_abstraction_with_bound_name_different() {
        assert_eq!(
            l("x", "z"),
            l("x", "y").substitute("y", &v("z"))
        )
    }

    #[test]
    fn test_abstraction_with_bound_name_same() {
        assert_eq!(
            l(prime("x"), a("y", "y")),
            l("x", a("x", "y")).substitute("x", &v("y"))
        )
    }

    #[test]
    fn test_abstraction_with_bound_name_in_free_variables_of_substitute() {
        assert_eq!(
            l(prime("x"), a(l("y", a("x", "y")), "y")),
            l("x", a("x", "y")).substitute("x", &l("y", a("x", "y")))
        )
    }

    mod rewrite {
        use super::prime;
        use super::super::*;
        use crate::parser::{l, a, v};

        #[test]
        fn test_with_variable() {
            assert_eq!(
                v("x"),
                rewrite("x", &v("x"))
            )
        }

        #[test]
        fn test_with_application() {
            assert_eq!(
                a("x", "y"),
                rewrite("x", &a("x", "y"))
            )
        }

        #[test]
        fn test_simple_abstraction() {
            assert_eq!(
                l(prime("x"), a(prime("x"), "y")),
                rewrite("x", &l("x", a("x", "y")))
            )
        }

        #[test]
        fn test_nested_abstraction_with_same_bound_variable_name() {
            let x_p = prime("x");
            let x_pp = prime(&prime("x"));
            assert_eq!(
                l(x_p.clone(), a(x_p.clone(), l("y", a("y", l(x_pp.clone(), x_pp.clone()))))),
                rewrite("x", &l("x", a("x", l("y", a("y", l("x", "x"))))))
            )
        }
    }

}
