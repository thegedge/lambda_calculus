#![feature(box_syntax, box_patterns)]

use std::io::{self, Read};

mod parser;
mod reduce;
mod vars;

fn main() -> Result<(), failure::Error> {
    let mut text = String::new();
    io::stdin().read_to_string(&mut text).unwrap();
    parser::parse(&text).map(|r| {
        println!("{:#?}", r);
    })
}
