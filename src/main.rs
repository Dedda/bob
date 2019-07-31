extern crate all_args_string;
extern crate string_stupidify;

use string_stupidify::*;
use std::env;
use string_stupidify::decorators::{Alternate, VaporWave, Shuffle, AlphaSort};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        help()
    } else {
        let (mut decorators, text) = parse_args();
        if decorators.is_empty() {
            decorators = vec![Box::new(Alternate)];
        }
        let text = text.decorate(&decorators).unwrap();
        println!("{}", text);
    }
}

fn help() {
    println!(
"Bob version {}
Usage:
  $ bob [flags...] [text]
Possible flags:
  --bob   Print chars in alternating case
  --rev   Reverse the text
  --vap   V A P O R W A V E
  --ran   Shuffle chars in text randomly
  --abc   Sort chars in text alphabetically
  --low   Convert to lower case
  --big   Convert to upper case
Flags can be combined and will be applied in the order they are set.",
    env!("CARGO_PKG_VERSION"));
}

fn parse_args() -> (Vec<Box<dyn StringDecorator>>, String) {
    let mut args: Vec<String> = env::args().collect();
    args.remove(0);
    let mut in_text = false;
    let mut text = vec![];
    let mut decorators = vec![];
    let mut current_dec: Option<String> = None;
    let mut current_decorator_args: Vec<String> = vec![];
    for arg in args {
        if !in_text {
            if arg.starts_with("--") {
                push_optional_decorator(&mut decorators, &current_dec, &current_decorator_args);
                current_dec = Some(arg.clone());
            } else if arg.starts_with("-") {
                current_decorator_args.push(arg.clone());
            } else {
                push_optional_decorator(&mut decorators, &current_dec, &current_decorator_args);
                in_text = true;
            }
        }
        if in_text {
            text.push(arg.clone());
        }
    }
    (decorators, text.join(" "))
}

fn push_optional_decorator(decorators: &mut Vec<Box<StringDecorator>>, name: &Option<String>, args: &Vec<String>) {
    match name {
        Some(dec) => {
            match decorator_from_args(&dec, &args) {
                Some(dec) => decorators.push(dec),
                None => {},
            }
        },
        None => {},
    }
}

fn decorator_from_args(name: &str, _args: &Vec<String>) -> Option<Box<dyn StringDecorator>> {
    match name {
        "--bob" => Some(Box::new(Alternate)),
        "--rev" => Some(Box::new(Reverse)),
        "--vap" => Some(Box::new(VaporWave)),
        "--ran" => Some(Box::new(Shuffle)),
        "--abc" => Some(Box::new(AlphaSort)),
        "--low" => Some(Box::new(LowerCase)),
        "--big" => Some(Box::new(UpperCase)),
        _ => None,
    }
}

struct Reverse;

impl StringDecorator for Reverse {
    fn decorate(&self, text: &String) -> Result<String, DecorationError> {
        Ok(text.chars().rev().collect())
    }
}

struct LowerCase;

impl StringDecorator for LowerCase {
    fn decorate(&self, text: &String) -> Result<String, DecorationError> {
        Ok(text.to_lowercase())
    }
}

struct UpperCase;

impl StringDecorator for UpperCase {
    fn decorate(&self, text: &String) -> Result<String, DecorationError> {
        Ok(text.to_uppercase())
    }
}