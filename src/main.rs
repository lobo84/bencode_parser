extern crate bencode_parser;

use bencode_parser::parser::parse;
use bencode_parser::parser::pp_bencodes;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::str;
use std::env;
use std::process::exit;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("usage: {} <file>", &args[0]);
        exit(1);
    }
    let path = Path::new(&args[1]);
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => panic!("could not open {}: {}", display, why.description()),
        Ok(file) => file,
    };

    let mut buffer: Vec<u8> = Vec::new();
    match file.read_to_end(&mut buffer) {
        Err(why) => panic!("could not read file {}", why.description()),
        Ok(size) => size,
    };

    let r = parse(&buffer);
    match r {
        Ok(o) => pp_bencodes(&o,0),
        _ => println!("failed"),
    }
}
