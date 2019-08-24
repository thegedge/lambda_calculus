#![feature(box_syntax, box_patterns)]

use std::io::{self, Read};

mod parser;
mod reduce;
mod substitution;
mod vars;

use reduce::{
    CallByValue as Strategy,
    Reduction,
};

fn main() -> Result<(), failure::Error> {
    let mut text = String::new();
    io::stdin().read_to_string(&mut text).unwrap();
    parser::parse(&text).map(|r| {
        println!(" original: {}", r);
        println!("evaluated: {}", Strategy::new().reduce(&r));
    })
}
