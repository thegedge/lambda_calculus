#![feature(box_syntax, box_patterns)]

extern crate pest;
#[macro_use] extern crate pest_derive;
#[macro_use] extern crate failure;

use std::io::{self, Read};

mod parser;

fn main() -> Result<(), failure::Error> {
    let mut text = String::new();
    io::stdin().read_to_string(&mut text).unwrap();
    parser::parse(&text).map(|r| {
        println!("{:#?}", r);
    })
}
