#![feature(box_syntax, box_patterns, bind_by_move_pattern_guards)]

use std::io::{self, Read};

mod notation;
mod parser;
mod reduce;
mod substitution;
mod vars;

use reduce::{
    CallByValue as Strategy,
    Reduction,
};

fn main() -> Result<(), failure::Error> {
    let mut parser = parser::Parser::new();

    let mut text = String::new();
    io::stdin().read_to_string(&mut text).unwrap();
    parser.parse(&text)
        .map(|result| {
            for term in result {
                println!(" original: {}", term);
                println!("evaluated: {}\n", Strategy::new().reduce(term));
            }
        })
}
