#![feature(nll)]

#[macro_use] extern crate lazy_static;
extern crate regex;
use regex::Regex;

extern crate aoclib;
use aoclib::*;

const GENERATOR_A_FACTOR : u32 = 16807;
const GENERATOR_B_FACTOR : u32 = 48271;
const GENERATE_MASK : u64 = 0x000000007FFFFFFF;
const COMPARISON_MASK : u64 = 0xffff;
const NUM_ROUNDS : u32 = 40000000;

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
        self.value = (self.value * (self.factor as u64)) % GENERATE_MASK;
        Some(self.value)
    }
}

fn count_matches(input : &str, num_rounds : usize) -> usize {
    let mut lines = input.lines();
    let mut gen_a = Generator::from(lines.next().unwrap(), GENERATOR_A_FACTOR);
    let mut gen_b = Generator::from(lines.next().unwrap(), GENERATOR_B_FACTOR);

    gen_a.zip(gen_b).take(num_rounds).inspect(|&(a, b)| {
        //eprintln!("a: {:x}, b: {:x}", a, b);
    }).filter(|&(a, b)| {
        (a & COMPARISON_MASK) == (b & COMPARISON_MASK)
    }).count()
}

fn solve_a(input : &str) -> usize {
    count_matches(input, NUM_ROUNDS as usize)
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
    fn count_matches_1() {
        let input =
r"Generator A starts with 65
Generator B starts with 8921";
        assert_eq!(count_matches(&input, 3), 1);
    }

    #[test]
    fn a_given() {
        let input =
r"Generator A starts with 65
Generator B starts with 8921";
        assert_eq!(solve_a(&input), 588);
    }

    #[test]
    fn b_1() {
        let input = "blah";
        assert_eq!(solve_b(&input), 0);
    }
}
