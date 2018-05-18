#![feature(nll)]

use std::collections::HashMap;

extern crate aoclib;
use aoclib::*;
use aoclib::onoffpixel::OnOffPixel;
use aoclib::direction::Direction;

type InfectionGrid = HashMap<(i32, i32), OnOffPixel>;

struct WormProgress {
    pixels : InfectionGrid,
    pos : (i32, i32),
    dir : Direction,
    num_activated : u32,
}

impl WormProgress {
    fn load(input : &str) -> WormProgress {
        let offset = ((input.lines().nth(0).unwrap().len() - 1) / 2) as i32;

        let mut ret = WormProgress {
            pixels : InfectionGrid::new(),
            pos : (0, 0),
            dir : Direction::Up,
            num_activated : 0,
        };

        for (y, line) in input.lines().enumerate() {
            for (x, ch) in line.chars().enumerate() {
                let value = OnOffPixel::parse(ch);
                if value == OnOffPixel::On {
                    // y has to be flipped
                    let pos = ((x as i32) - offset, offset - (y as i32));
                    eprintln!("on at {:?}", pos);
                    ret.pixels.insert(pos, value);
                }
            }
        }

        ret
    }

    fn toggle_current(&mut self) -> bool {
        let ret;
        if let Some(val) = self.pixels.get_mut(&self.pos) {
            ret = val.is_on();
            *val = val.opposite();
        } else {
            ret = false;
            self.pixels.insert(self.pos, OnOffPixel::On);
        }
        ret
    }
}

impl Iterator for WormProgress {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        // if was infected
        if self.toggle_current() {
            self.dir = self.dir.turn_right();
        }

        // else was not infected
        else {
            //eprintln!("infecting {:?}", self.pos);
            self.dir = self.dir.turn_left();
            self.num_activated += 1;
        }

        let offset = self.dir.step_offset();
        self.pos = (self.pos.0 + offset.0, self.pos.1 + offset.1);

        Some(self.num_activated)
    }
}

fn count_infected(input : &str, iterations : usize) -> u32 {
    let worm = WormProgress::load(input);
    worm.take(iterations).last().unwrap()
}

fn solve_a(input : &str) -> u32 {
    count_infected(input, 10000)
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
        assert_eq!(count_infected(&input, 1), 1);
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
