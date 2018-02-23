#![feature(nll)]

#[macro_use] extern crate lazy_static;
extern crate regex;
use regex::Regex;

extern crate aoclib;
use aoclib::*;

const GENERATOR_A_FACTOR : u32 = 16807;
const GENERATOR_B_FACTOR : u32 = 48271;
const MASK : u64 = 0x000000007FFFFFFF;

struct Generator {
    value : u64,
    factor : u32,
}

impl Generator {
    fn new(value : u64, factor : u32) -> Generator {
        Generator { value, factor }
    }

    fn from(input : &str, factor : u32) -> Generator {
        lazy_static! {
            static ref RE_GENERATOR_SEED : regex::Regex = Regex::new(r"^Generator \w+ starts with (\d+)$").expect("failed to compile regex");
        }

        let generator_captures = RE_GENERATOR_SEED.captures_iter(input).nth(0).unwrap();

        Generator {
            value : generator_captures.get(1).unwrap().as_str().parse::<u64>().unwrap(),
            factor : factor,
        }
    }
}

impl Iterator for Generator {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        self.value = (self.value * (self.factor as u64)) & MASK;
        Some(self.value)
    }
}

fn solve_a(input : &str) -> u32 {
    let generators : Vec<Generator> = input.lines().enumerate().map(|(i, line)| {
        Generator::from(line, if i == 0 { GENERATOR_A_FACTOR } else { GENERATOR_B_FACTOR })
    }).collect();
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
        let input =
r"Generator A starts with 65
Generator B starts with 8921";
        assert_eq!(solve_a(&input), 0);
    }

    #[test]
    fn b_1() {
        let input = "blah";
        assert_eq!(solve_b(&input), 0);
    }
}
