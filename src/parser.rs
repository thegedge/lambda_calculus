use std::fmt;

use pest::{
    Parser,
    iterators::Pair,
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

#[derive(PartialEq)]
pub enum Term<'s> {
    Variable(&'s str),
    Abstraction(&'s str, Box<Term<'s>>),
    Application(Box<Term<'s>>, Box<Term<'s>>),
}

impl <'s> fmt::Display for Term<'s> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Term::Variable(s) => write!(f, "{}", s),
            Term::Abstraction(s, t) => write!(f, "λ{}.({})", s, t),
            Term::Application(box Term::Variable(t1), box Term::Variable(t2)) => write!(f, "{} {}", t1, t2),
            Term::Application(box Term::Variable(t1), t2) => write!(f, "{} ({})", t1, t2),
            Term::Application(t1, box Term::Variable(t2)) => write!(f, "({}) {}", t1, t2),
            Term::Application(t1, t2) => write!(f, "({}) ({})", t1, t2),
        }
    }
}

impl <'s> fmt::Debug for Term<'s> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

pub fn parse<'s>(text: &'s str) -> Result<Term<'s>, failure::Error> {
    let pair = LambdaCalculusParser::parse(Rule::main, &text)
        .map_err(|e| ParseError::PestError(e))
        .map(|mut pairs| pairs.next())?;

    let result = pest_to_term(pair.ok_or(ParseError::EmptyInput)?)?;
    Ok(result)
}

fn pest_to_term(pair: Pair<Rule>) -> Result<Term, ParseError> {
    match pair.as_rule() {
        Rule::main | Rule::term => {
            pest_to_term(pair.into_inner().next().ok_or(ParseError::Unknown("main|term|simple_term"))?)
        },
        Rule::variable => {
            Ok(Term::Variable(pair.as_str()))
        },
        Rule::abstraction => {
            let mut pairs = pair.into_inner();
            Ok(Term::Abstraction(
                pairs.next().map(|p| p.as_str()).ok_or(ParseError::Unknown("abstraction[0]"))?,
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
    use super::Term::*;

    fn l<'s>(name: &'s str, body: Term<'s>) -> Term<'s> { Abstraction(name, Box::new(body)) }
    fn a<'s>(a: Term<'s>, b: Term<'s>) -> Term<'s> { Application(Box::new(a), Box::new(b)) }
    fn v<'s>(name: &'s str) -> Term<'s> { Variable(name) }

    fn assert_eq<'s>(code: &'s str, expected: Term<'s>) {
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
            l("x", v("x"))
        );
    }

    #[test]
    pub fn test_parses_application_without_parentheses() {
        assert_eq(
            r"x y z",
            a(v("x"), a(v("y"), v("z")))
        );
    }

    #[test]
    pub fn test_parses_application_with_parentheses() {
        assert_eq(
            r"(x y) z",
            a(a(v("x"), v("y")), v("z"))
        );
    }

    #[test]
    pub fn test_parses_with_correct_associativity1() {
        assert_eq(
            r"\x . \y . (x x) y",
            l("x", l("y", a(a(v("x"), v("x")), v("y"))))
        );
    }

    #[test]
    pub fn test_parses_with_correct_associativity2() {
        assert_eq(
            r"\x . \y . x (x y)",
            l("x", l("y", a(v("x"), a(v("x"), v("y")))))
        );
    }

    #[test]
    pub fn test_parses_with_correct_associativity3() {
        assert_eq(
            r"\x . (\y . x x y) x",
            l("x", a(l("y", a(v("x"), a(v("x"), v("y")))), v("x"))),
        );
    }
}
