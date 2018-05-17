#![feature(nll)]

use std::collections::HashSet;
use std::hash::{Hash, Hasher};

extern crate aoclib;
use aoclib::*;
use aoclib::onoffpixel::OnOffPixel;

struct PixelInGrid {
    pos : (i32, i32),
    value : OnOffPixel,
}

struct WormProgress {
    pixels : HashSet<PixelInGrid>,
    pos : (i32, i32),
    num_activated : u32,
}

impl PartialEq for PixelInGrid {
    fn eq(&self, other: &PixelInGrid) -> bool {
        self.pos == other.pos
    }
}

impl Eq for PixelInGrid {}

impl Hash for PixelInGrid {
    fn hash<H>(&self, state : &mut H)
    where H : Hasher {
        self.pos.hash(state)
    }
}

impl WormProgress {
    fn load(input : &str) -> WormProgress {
        let offset = ((input.lines().nth(0).unwrap().len() / 2) - 1) as i32;
        let mut pixels = HashSet::new();

        for (y, line) in input.lines().enumerate() {
            for (x, ch) in line.chars().enumerate() {
                let value = OnOffPixel::parse(ch);
                if value == OnOffPixel::On {
                    pixels.insert(PixelInGrid {
                        pos : (offset - (x as i32), (offset - (y as i32))),
                        value,
                    });
                }
            }
        }

        WormProgress {
            pixels,
            pos : (0, 0),
            num_activated : 0,
        }
    }
}

fn count_infected(input : &str, iterations : usize) -> usize {
    0
}

fn solve_a(input : &str) -> u32 {
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
    fn a_given() {
        let input =
r"..#
#..
...";
        assert_eq!(count_infected(&input, 0), 0);
        assert_eq!(count_infected(&input, 7), 5);
        assert_eq!(count_infected(&input, 70), 41);
        assert_eq!(solve_a(&input), 5587);
    }

    #[test]
    fn b_1() {
        let input = "blah";
        assert_eq!(solve_b(&input), 0);
    }
}
