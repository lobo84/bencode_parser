use nom::*;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::collections::HashMap;
use std::str;
use std::env;
use std::process::exit;

#[derive(Clone,Debug,PartialEq)]
pub enum Bencode {
    BInt(i64),
    BString(Vec<u8>),
    BList(Vec<Bencode>),
    BDict(HashMap<Vec<u8>, Bencode>),
}

fn u8toi64(b: &[u8]) -> i64 {
    str::from_utf8(b).unwrap().parse::<i64>().unwrap()
}

named!(bint<&[u8], i64>,
       chain!(tag!("i") ~
              bytes: is_not!("e") ~
              tag!("e"),
              || {u8toi64(bytes)}
              )
       );

named!(bstring<&[u8], (Vec<u8>)>,
       chain!(n: digit ~
              tag!(":") ~
              bytes: take!(u8toi64(n)),
              || {bytes.to_vec()}
              )
       );

named!(blist<&[u8], (Vec<Bencode>)>,
       chain!(tag!("l") ~
              bencodes: bencodes ~
              tag!("e"),
              || {bencodes}
              )
       );

fn pairs_to_map(pairs: &Vec<(Vec<u8>,Bencode)>) -> HashMap<Vec<u8>, Bencode> {
    let mut h: HashMap<Vec<u8>, Bencode> = HashMap::new();
    for &(ref k, ref v) in pairs {
        h.insert(k.clone(),v.clone());
    }
    return h;
}

named!(bdict<&[u8], (HashMap<Vec<u8>,Bencode>)>,
       chain!(tag!("d") ~
              kvs: many0!(pair!(bstring, bencode)) ~
              tag!("e"),
              || {pairs_to_map(&kvs)}
              )
       );

named!(bencodes<&[u8], (Vec<Bencode>)>,
       many0!(bencode)
       );

named!(bencode<&[u8], Bencode>,
       alt!(
           map!(bint, Bencode::BInt) |
           map!(bstring, Bencode::BString) |
           map!(blist, Bencode::BList) |
           map!(bdict, Bencode::BDict)
           )
       );

fn pp_bstring(s: &Vec<u8>, level: u8) {
    match str::from_utf8(s) {
        Ok(r) => println!("{}{}",indent(level),r),
        Err(_) => println!("{}[{} bytes]" ,indent(level),s.len()),
    }
}

fn pp_dict(m: &HashMap<Vec<u8>, Bencode>, level: u8) {
    for (ref k,ref v) in m {
        println!("{}{} =>", indent(level),str::from_utf8(k).unwrap());
        pp_bencode(v, level+1);
    }
}

fn pp_bencode(b: &Bencode, level: u8) {
    match b {
        &Bencode::BInt(i) => println!("{}{}",indent(level),i),
        &Bencode::BString(ref s) => pp_bstring(s,level),
        &Bencode::BList(ref l) => pp_bencodes(l,level),
        &Bencode::BDict(ref m) => pp_dict(m,level),
    }
}

fn indent(n: u8) -> String {
    let mut s = String::new();
    for _ in 0..n {
        s.push(' ');
    }
    return s;
}

pub fn pp_bencodes(bs: &Vec<Bencode>, level: u8) {
    for b in bs {
        println!("{}[",indent(level));
        pp_bencode(b, level+1);
        println!("{}]",indent(level));
    }
}

#[test]
fn test_bint() {
    match bint(b"i-132e") {
        IResult::Done(_, o) => assert_eq!(o, -132),
        _ => assert!(false),
    }
}

#[test]
fn test_bstring() {
    match bstring(b"3:apa") {
        IResult::Done(_, o) => assert_eq!(o, b"apa"),
        _ => assert!(false),
    }
}

#[test]
fn test_blist() {
    let e = [Bencode::BString(b"apa".to_vec()),
             Bencode::BInt(32),
             Bencode::BInt(43)];
    match blist(b"l3:apai32ei43ee") {
        IResult::Done(i,o) => assert_eq!(o,e),
        _ => assert!(false),
    }
}

#[test]
fn test_bdict() {
    let mut m: HashMap<Vec<u8>,Bencode> = HashMap::new();
    m.insert(b"apa".to_vec(), Bencode::BInt(10));
    match bdict(b"d3:apai10ee") {
        IResult::Done(i,o) => assert_eq!(o,m),
        _ => assert!(false),
    }
}

#[test]
fn test_bencodes() {
    let e = [Bencode::BString(b"apa".to_vec()), Bencode::BInt(-23)];
    let r = bencodes(b"3:apai-23e");
    match r {
        IResult::Done(_, o) => assert_eq!(o,e),
        _ => assert!(false),
    }
}

pub fn parse(bytes: &[u8]) -> Result<(Vec<Bencode>), String> {
    return match bencodes(bytes) {
        IResult::Done(_,o) => Result::Ok(o),
        IResult::Error(_) => Result::Err(String::from("Error")),
        _ => Err(String::from("Error")),
    }
}
