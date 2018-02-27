#![feature(nll)]

use std::fmt;

#[macro_use] extern crate lazy_static;
extern crate regex;
use regex::Regex;

extern crate aoclib;
use aoclib::*;

enum DanceMove<'t> {
    Spin(u32),
    Exchange(u32, u32),
    Partner(&'t str, &'t str),
}

struct Dance<'t> {
    moves : Vec<DanceMove<'t>>,
}

struct Performance<'t> {
    dancers : Vec<String>,
    step : std::slice::Iter<'t, DanceMove<'t>>,
}

impl<'t> DanceMove<'t> {
    fn from(input : &'t str) -> DanceMove<'t> {
        lazy_static! {
            static ref RE_SPIN : regex::Regex = Regex::new(r"^s(\d+)$").expect("failed to compile regex");
            static ref RE_EXCHANGE : regex::Regex = Regex::new(r"^x(\d+)/(\d+)$").expect("failed to compile regex");
            static ref RE_PARTNER : regex::Regex = Regex::new(r"^p(\w+)/(\w+)$").expect("failed to compile regex");
        }

        if let Some(captures) = RE_SPIN.captures_iter(input).next() {
            DanceMove::Spin(captures.get(1).unwrap().as_str().parse::<u32>().unwrap())
        } else if let Some(captures) = RE_EXCHANGE.captures_iter(input).next() {
            DanceMove::Exchange(captures.get(1).unwrap().as_str().parse::<u32>().unwrap(), captures.get(2).unwrap().as_str().parse::<u32>().unwrap())
        } else if let Some(captures) = RE_PARTNER.captures_iter(input).next() {
            DanceMove::Partner(captures.get(1).unwrap().as_str(), captures.get(2).unwrap().as_str())
        } else {
            panic!("invalid move {}", input);
        }
    }
}

impl<'t> fmt::Display for DanceMove<'t> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &DanceMove::Spin(a) => write!(f, "s{}", a),
            &DanceMove::Exchange(a, b) => write!(f, "x{}/{}", a, b),
            &DanceMove::Partner(a, b) => write!(f, "p{}/{}", a, b),
        }
    }
}

impl<'t> Dance<'t> {
    fn new(moves : &'t str) -> Dance<'t> {
        Dance {
            moves : moves.lines().map(DanceMove::from).collect(),
        }
    }

    fn perform<'a>(&'a self, num_dancers : u8) -> Performance<'a> {
        Performance {
            dancers : (0 .. num_dancers).map(|i| {
                String::from(((('a' as u8) + i) as char).to_string())
            }).collect(),
            step : self.moves.iter(),
        }
    }
}

impl<'t> Iterator for Dance<'t> {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}

fn solve_a(input : &str) -> u32 {
    let dance = Dance::new(input);
    0
}

fn solve_b(input : &str) -> u32 {
    0
}

fn main() {
    let input = read_all_stdin();
    //eprintln!("input: {}", input);

    if aoclib::should_solve_puzzle_a() {
        println!("answer: {}", solve_a(&input));
    } else {
        println!("answer: {}", solve_b(&input));
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn a_1() {
        let input = "blah";
        assert_eq!(solve_a(&input), 0);
    }

    #[test]
    fn b_1() {
        let input = "blah";
        assert_eq!(solve_b(&input), 0);
    }
}
