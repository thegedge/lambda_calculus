#![feature(box_syntax, box_patterns, bind_by_move_pattern_guards)]

use std::io::{self, Read};

mod evaluation;
mod parser;
mod substitution;
mod term;
mod vars;

use evaluation::{
    Normal as Strategy,
    EmptyContext,
    Evaluable,
};

fn main() -> Result<(), failure::Error> {
    let mut parser = parser::Parser::new();

    let mut text = String::new();
    io::stdin().read_to_string(&mut text).unwrap();
    parser
        .parse(&text)
        .map(|result| {
            for term in result {
                // TODO maybe have parser terms maintain the slice from which they came, so we
                // could write something like (expr = result)
                println!("{}", Strategy::new().evaluate(&mut EmptyContext{}, term));
            }
        })
}
