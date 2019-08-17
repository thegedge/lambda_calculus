use pest::{
    Parser,
    iterators::Pairs,
};

#[derive(Parser)]
#[grammar = "grammar.pest"]
struct LambdaCalculusParser;

#[derive(Debug, Fail)]
enum ParseError {
    #[fail(display = "failed to parse: {}", _0)]
    PestError(pest::error::Error<Rule>),
}

pub fn parse(text: &str) -> Result<Pairs<Rule>, failure::Error> {
    Ok(
        LambdaCalculusParser::parse(Rule::main, &text)
            .map_err(|e| ParseError::PestError(e))?
    )
}
