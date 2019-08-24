use std::{
    collections::HashMap,
    fmt,
};

use pest::{
    Parser,
    iterators::Pair
};

use pest_derive::Parser;

use failure::{
    Fail,
};

#[derive(Parser)]
#[grammar = "grammar.pest"]
struct LambdaCalculusParser<'p> {
    macros: HashMap<&'p str, Term>,
}

#[derive(Debug, Fail)]
enum ParseError {
    #[fail(display = "empty input")]
    EmptyInput,

    #[fail(display = "failed to parse: {}", _0)]
    PestError(pest::error::Error<Rule>),

    #[fail(display = "parse error for pair: {}", _0)]
    Unknown(&'static str),
}

#[derive(Clone, PartialEq)]
pub enum Term {
    Variable(String),
    Abstraction(String, Box<Term>),
    Application(Box<Term>, Box<Term>),
}

impl fmt::Display for Term {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Term::Variable(s) => write!(f, "{}", s),
            Term::Abstraction(s, t) => write!(f, "λ{}.{}", s, t),
            Term::Application(box Term::Variable(t1), box Term::Variable(t2)) => write!(f, "{} {}", t1, t2),
            Term::Application(box Term::Variable(t1), t2) => write!(f, "{} ({})", t1, t2),
            Term::Application(t1, box Term::Variable(t2)) => write!(f, "({}) {}", t1, t2),
            Term::Application(t1, t2) => write!(f, "({}) ({})", t1, t2),
        }
    }
}

impl fmt::Debug for Term {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Term::Variable(s) => write!(f, "{:?}", s),
            Term::Abstraction(s, t) => write!(f, "l({:?}, {:?})", s, t),
            Term::Application(t1, t2) => write!(f, "a({:?}, {:?})", t1, t2),
        }
    }
}

pub fn l<S, T>(name: S, body: T) -> Term
    where S: Into<String>,
          T: Into<Term>
{
    Term::Abstraction(name.into(), Box::new(body.into()))
}

pub fn a<T1, T2>(a: T1, b: T2) -> Term
    where T1: Into<Term>,
          T2: Into<Term>
{
    Term::Application(Box::new(a.into()), Box::new(b.into()))
}

pub fn v<T: Into<String>>(name: T) -> Term {
    Term::Variable(name.into())
}

impl <T> From<T> for Term
    where T: Into<String>
{
    fn from(s: T) -> Term {
        Term::Variable(s.into())
    }
}

pub fn parse(text: &str) -> Result<Term, failure::Error> {
    let pair = LambdaCalculusParser::parse(Rule::main, &text)
        .map_err(|e| ParseError::PestError(e))
        .map(|mut pairs| pairs.next())?;

    let result = LambdaCalculusParser::new().pest_to_term(pair.ok_or(ParseError::EmptyInput)?)?;
    Ok(result)
}

impl <'p> LambdaCalculusParser<'p> {
    fn new() -> LambdaCalculusParser<'p> {
        LambdaCalculusParser {
            macros: HashMap::new(),
        }
    }

    fn pest_to_term(&mut self, pair: Pair<'p, Rule>) -> Result<Term, ParseError> {
        match pair.as_rule() {
            Rule::main => {
                let mut pairs = pair.into_inner();
                pairs.next()
                     .ok_or(ParseError::Unknown("main"))
                     .and_then(|t| self.read_macros(t))?;

                self.pest_to_term(pairs.next().ok_or(ParseError::Unknown("main|term|simple_term"))?)
            },
            Rule::term | Rule::simple_term => {
                self.pest_to_term(pair.into_inner().next().ok_or(ParseError::Unknown("term"))?)
            },
            Rule::variable => {
                let name = pair.as_str();
                Ok(
                    self.macros.get(name)
                               .map(|t| t.clone())
                               .unwrap_or_else(|| Term::Variable(name.to_string()))
                )
            },
            Rule::abstraction => {
                let mut pairs = pair.into_inner();
                Ok(Term::Abstraction(
                    pairs.next().map(|p| p.as_str().to_string()).ok_or(ParseError::Unknown("abstraction[0]"))?,
                    Box::new(
                        pairs.next()
                             .ok_or(ParseError::Unknown("abstraction[1]"))
                             .map(|t| self.pest_to_term(t))??
                    )
                ))
            },
            Rule::application => {
                let mut pairs = pair.into_inner();
                let t1 =
                        pairs.next()
                             .ok_or(ParseError::Unknown("application[0]"))
                             .map(|t| self.pest_to_term(t))??;

                pairs.try_fold(t1, |app, t| {
                    let term = self.pest_to_term(t)?;
                    Ok(Term::Application(box app, box term))
                })
            },
            rule => unreachable!("{:?}", rule),
        }
    }

    fn read_macros(&mut self, pair: Pair<'p, Rule>) -> Result<(), ParseError> {
        Ok(for macro_term in pair.into_inner() {
            let mut pairs = macro_term.into_inner();
            pairs.next()
                 .ok_or(ParseError::Unknown("macro name"))
                 .map(|t| t.as_str())
                 .and_then(|name| {
                     let macro_body = self.pest_to_term(pairs.next().ok_or(ParseError::Unknown("macro body"))?)?;
                     self.macros.insert(name, macro_body);
                     Ok(())
                 })?;
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_eq(code: &str, expected: Term) {
        assert_eq!(expected, parse(code).unwrap(), "input program: {}", code);
    }

    #[test]
    pub fn test_parses_variable() {
        assert_eq(
            r"x",
            v("x")
        );
    }

    #[test]
    pub fn test_parses_abstraction() {
        assert_eq(
            r"\x    . x",
            l("x", "x")
        );
    }

    #[test]
    pub fn test_parses_applications_without_parentheses_are_left_associative() {
        assert_eq(
            r"x y z",
            a(a("x", "y"), "z")
        );
    }

    #[test]
    pub fn test_parses_application_with_parentheses() {
        assert_eq(
            r"x (y z)",
            a("x", a("y", "z"))
        );
    }

    #[test]
    pub fn test_parses_with_correct_associativity1() {
        assert_eq(
            r"\x . \y . (x x) y",
            l("x", l("y", a(a("x", "x"), "y")))
        );
    }

    #[test]
    pub fn test_parses_with_correct_associativity2() {
        assert_eq(
            r"\x . \y . x (x y)",
            l("x", l("y", a("x", a("x", "y"))))
        );
    }

    #[test]
    pub fn test_parses_with_correct_associativity3() {
        assert_eq(
            r"\x . (\y . x x y) x",
            l("x", a(l("y", a(a("x", "x"), "y")), "x")),
        );
    }

    #[test]
    pub fn test_parses_with_correct_associativity4() {
        assert_eq(
            r"(\x . \y . x x y) x",
            a(l("x", l("y", a(a("x", "x"), "y"))), "x")
        );
    }

    #[test]
    pub fn test_parses_with_complex_application() {
        assert_eq(
            r"(\y.y z) \z.z",
            a(l("y", a("y", "z")), l("z", "z"))
        );
    }

    #[test]
    pub fn test_parses_with_whitespace_at_end() {
        assert_eq(
            "\\x . \\y . (x x) y  \t \r  \n",
            l("x", l("y", a(a("x", "x"), "y")))
        );
    }

    #[test]
    pub fn test_parses_and_substitutes_macro() {
        assert_eq(
            r"id = \x.x; id (id y)",
            a(l("x", "x"), a(l("x", "x"), "y"))
        );
    }
}
