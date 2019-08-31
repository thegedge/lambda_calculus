use std::{
    collections::HashMap,
    fmt,
};

use pest::{
    Parser as PestParser,
    iterators::Pair
};

use failure::{
    Fail,
};

#[derive(pest_derive::Parser)]
#[grammar = "grammar.pest"]
struct LambdaCalculusParser;

pub struct Parser<'p> {
    macros: HashMap<&'p str, Term>,
    terms: Vec<Term>,
}

#[derive(Debug, Fail)]
enum ParseError {
    #[fail(display = "empty input")]
    EmptyInput,

    #[fail(display = "no terms")]
    NoTerms,

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

impl Term {
    pub fn is_redex(&self) -> bool {
        match self {
            Term::Application(box Term::Abstraction(_, _), _) => true,
            Term::Application(t, _) => t.is_redex(),
            _ => false,
        }
    }
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

pub fn parse(text: &str) -> Result<Vec<Term>, failure::Error> {
    let mut parser = Parser::new();
    parser.parse(text).map(Iterator::collect)
}

pub fn parse_one(text: &str) -> Result<Term, failure::Error> {
    let mut parser = Parser::new();
    parser.parse(text).and_then(|mut iter| iter.next().ok_or(ParseError::NoTerms.into()))
}

impl <'p> Parser<'p> {
    pub fn new() -> Parser<'p> {
        Parser {
            macros: HashMap::new(),
            terms: Vec::new(),
        }
    }

    pub fn parse(&mut self, text: &'p str) -> Result<impl Iterator<Item=Term> + '_, failure::Error> {
        let pair = LambdaCalculusParser::parse(Rule::main, &text)
            .map_err(|e| ParseError::PestError(e))
            .map(|mut pairs| pairs.next())?
            .ok_or(ParseError::EmptyInput)?;

        self.process_pair(pair);
        Ok(self.terms.drain(0..))
    }

    fn process_pair(&mut self, pair: Pair<'p, Rule>) -> Result<(), ParseError> {
        match pair.as_rule() {
            Rule::main  => {
                pair.into_inner().map(|p| self.process_pair(p)).collect()
            },
            Rule::macro_ => {
                self.read_macro(pair)
            },
            Rule::term => {
                let term = self.process_term(pair)?;
                self.terms.push(term);
                Ok(())
            },
            Rule::EOI => {
                Ok(())
            },
            rule => unreachable!("{:?}", rule),
        }
    }

    fn process_term(&mut self, pair: Pair<'p, Rule>) -> Result<Term, ParseError> {
        match pair.as_rule() {
            Rule::term | Rule::simple_term => {
                self.process_term(pair.into_inner().next().ok_or(ParseError::Unknown("term"))?)
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
                             .map(|t| self.process_term(t))??
                    )
                ))
            },
            Rule::application => {
                let mut pairs = pair.into_inner();
                let t1 =
                        pairs.next()
                             .ok_or(ParseError::Unknown("application[0]"))
                             .map(|t| self.process_term(t))??;

                pairs.try_fold(t1, |app, t| {
                    let term = self.process_term(t)?;
                    Ok(Term::Application(box app, box term))
                })
            },
            rule => unreachable!("{:?}", rule),
        }
    }

    fn read_macro(&mut self, pair: Pair<'p, Rule>) -> Result<(), ParseError> {
        let mut pairs = pair.into_inner();
        pairs.next()
             .ok_or(ParseError::Unknown("macro name"))
             .map(|t| t.as_str())
             .and_then(|name| {
                 let macro_body = self.process_term(pairs.next().ok_or(ParseError::Unknown("macro body"))?)?;
                 self.macros.insert(name, macro_body);
                 Ok(())
             })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_eq(code: &str, expected: Term) {
        assert_eq!(
            expected,
            parse_one(code).unwrap(),
            "input program: {}",
            code
        );
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
    pub fn test_parses_with_empty_lines() {
        assert_eq(
            "a = b;\n\na",
            v("b"),
        );
    }

    #[test]
    pub fn test_parses_and_substitutes_macro() {
        assert_eq(
            r"id = \x.x; id (id y)",
            a(l("x", "x"), a(l("x", "x"), "y"))
        );
    }

    #[test]
    pub fn test_is_redex_true_for_abstraction_application() {
        assert!(
            a(l("x", "x"), "y").is_redex()
        );
    }

    #[test]
    pub fn test_is_redex_false_for_non_redex() {
        assert!(
            !a("y", l("x", "x")).is_redex()
        );
    }

    #[test]
    pub fn test_is_redex_true_for_nested_abstraction() {
        assert!(
            a(a(a(l("x", "x"), "a"), "b"), "c").is_redex()
        );
    }
}
