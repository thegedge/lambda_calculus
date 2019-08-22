use std::fmt;

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
struct LambdaCalculusParser;

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

pub fn parse(text: &str) -> Result<Term, failure::Error> {
    let pair = LambdaCalculusParser::parse(Rule::main, &text)
        .map_err(|e| ParseError::PestError(e))
        .map(|mut pairs| pairs.next())?;

    let result = pest_to_term(pair.ok_or(ParseError::EmptyInput)?)?;
    Ok(result)
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

fn pest_to_term(pair: Pair<Rule>) -> Result<Term, ParseError> {
    match pair.as_rule() {
        Rule::main | Rule::term => {
            pest_to_term(pair.into_inner().next().ok_or(ParseError::Unknown("main|term|simple_term"))?)
        },
        Rule::variable => {
            Ok(Term::Variable(pair.as_str().to_string()))
        },
        Rule::abstraction => {
            let mut pairs = pair.into_inner();
            Ok(Term::Abstraction(
                pairs.next().map(|p| p.as_str().to_string()).ok_or(ParseError::Unknown("abstraction[0]"))?,
                Box::new(pairs.next().ok_or(ParseError::Unknown("abstraction[1]")).map(pest_to_term)??)
            ))
        },
        Rule::application => {
            let mut pairs = pair.into_inner();
            Ok(Term::Application(
                Box::new(pairs.next().ok_or(ParseError::Unknown("application[0]")).map(pest_to_term)??),
                Box::new(pairs.next().ok_or(ParseError::Unknown("application[1]")).map(pest_to_term)??)
            ))
        },
        rule => unreachable!("{:?}", rule),
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
    pub fn test_parses_application_without_parentheses() {
        assert_eq(
            r"x y z",
            a("x", a("y", "z"))
        );
    }

    #[test]
    pub fn test_parses_application_with_parentheses() {
        assert_eq(
            r"(x y) z",
            a(a("x", "y"), "z")
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
            l("x", a(l("y", a("x", a("x", "y"))), "x")),
        );
    }

    #[test]
    pub fn test_parses_with_correct_associativity4() {
        assert_eq(
            r"(\x . \y . x x y) x",
            a(l("x", l("y", a("x", a("x", "y")))), "x")
        );
    }

    #[test]
    pub fn test_parses_with_complex_application() {
        assert_eq(
            r"(\y.y z) \z.z",
            a(l("y", a("y", "z")), l("z", "z"))
        );
    }
}
