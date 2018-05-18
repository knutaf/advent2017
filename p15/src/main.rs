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
const NUM_ROUNDS_A : u32 = 40000000;
const NUM_ROUNDS_B : u32 = 5000000;

struct Generator {
    value : u64,
    factor : u32,
}

impl Generator {
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

fn count_matches(gen_a : impl Iterator<Item = u64>, gen_b : impl Iterator<Item = u64>, num_rounds : usize) -> usize {
    gen_a.zip(gen_b).take(num_rounds).inspect(|&(_a, _b)| {
        //eprintln!("a: {:x}, b: {:x}", _a, _b);
    }).filter(|&(a, b)| {
        (a & COMPARISON_MASK) == (b & COMPARISON_MASK)
    }).count()
}

fn count_matches_a(input : &str, num_rounds : usize) -> usize {
    let mut lines = input.lines();
    let gen_a = Generator::from(lines.next().unwrap(), GENERATOR_A_FACTOR);
    let gen_b = Generator::from(lines.next().unwrap(), GENERATOR_B_FACTOR);
    count_matches(gen_a, gen_b, num_rounds)
}

fn count_matches_b(input : &str, num_rounds : usize) -> usize {
    fn make_mod_filter(mod_by : u64) -> impl Fn(&u64)->bool {
        move |num : &u64| -> bool {
            (num % mod_by) == 0
        }
    }

    let mut lines = input.lines();
    let gen_a = Generator::from(lines.next().unwrap(), GENERATOR_A_FACTOR).filter(make_mod_filter(4));
    let gen_b = Generator::from(lines.next().unwrap(), GENERATOR_B_FACTOR).filter(make_mod_filter(8));

    count_matches(gen_a, gen_b, num_rounds)
}

fn solve_a(input : &str) -> usize {
    count_matches_a(input, NUM_ROUNDS_A as usize)
}

fn solve_b(input : &str) -> usize {
    count_matches_b(input, NUM_ROUNDS_B as usize)
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
    fn count_matches_a_1() {
        let input =
r"Generator A starts with 65
Generator B starts with 8921";
        assert_eq!(count_matches_a(&input, 2), 0);
        assert_eq!(count_matches_a(&input, 3), 1);
    }

    #[test]
    fn count_matches_b_1() {
        let input =
r"Generator A starts with 65
Generator B starts with 8921";
        assert_eq!(count_matches_b(&input, 1055), 0);
        assert_eq!(count_matches_b(&input, 1056), 1);
    }

    #[test]
    fn a_given() {
        let input =
r"Generator A starts with 65
Generator B starts with 8921";
        assert_eq!(solve_a(&input), 588);
    }

    #[test]
    fn b_given() {
        let input =
r"Generator A starts with 65
Generator B starts with 8921";
        assert_eq!(solve_b(&input), 309);
    }
}
