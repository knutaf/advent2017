#![feature(nll)]

use std::collections::HashMap;

extern crate aoclib;
use aoclib::*;
use aoclib::onoffpixel::OnOffPixel;
use aoclib::direction::Direction;

#[derive(PartialEq, Clone, Debug)]
enum InfectionState {
    Clean,
    Weakened,
    Infected,
    Flagged,
}

type InfectionGrid = HashMap<(i32, i32), InfectionState>;

struct WormProgress {
    pixels : InfectionGrid,
    pos : (i32, i32),
    dir : Direction,
    num_infected : u32,
    puzzle_a : bool,
}

impl WormProgress {
    fn load(input : &str, puzzle_a : bool) -> WormProgress {
        let offset = ((input.lines().nth(0).unwrap().len() - 1) / 2) as i32;

        let mut ret = WormProgress {
            pixels : InfectionGrid::new(),
            pos : (0, 0),
            dir : Direction::Up,
            num_infected : 0,
            puzzle_a,
        };

        for (y, line) in input.lines().enumerate() {
            for (x, ch) in line.chars().enumerate() {
                let value = OnOffPixel::parse(ch);
                if value == OnOffPixel::On {
                    // y has to be flipped
                    let pos = ((x as i32) - offset, offset - (y as i32));
                    eprintln!("on at {:?}", pos);
                    ret.pixels.insert(pos, InfectionState::Infected);
                }
            }
        }

        ret
    }

    fn advance_a(&mut self) -> InfectionState {
        let ret;
        if let Some(val) = self.pixels.get_mut(&self.pos) {
            ret = val.clone();
            if *val == InfectionState::Infected {
                *val = InfectionState::Clean;
            } else {
                *val = InfectionState::Infected;
            }
        } else {
            ret = InfectionState::Clean;
            self.pixels.insert(self.pos, InfectionState::Infected);
        }
        ret
    }

    fn advance_b(&mut self) -> InfectionState {
        let ret;
        if let Some(val) = self.pixels.get_mut(&self.pos) {
            ret = val.clone();

            *val = match val {
                &mut InfectionState::Clean => InfectionState::Weakened,
                &mut InfectionState::Weakened => InfectionState::Infected,
                &mut InfectionState::Infected => InfectionState::Flagged,
                &mut InfectionState::Flagged => InfectionState::Clean,
            };
        } else {
            ret = InfectionState::Clean;
            self.pixels.insert(self.pos, InfectionState::Weakened);
        }
        ret
    }
}

impl Iterator for WormProgress {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.puzzle_a {
            if self.advance_a() == InfectionState::Infected {
                self.dir = self.dir.turn_right();
            }

            // else was not infected
            else {
                //eprintln!("infecting {:?}", self.pos);
                self.dir = self.dir.turn_left();
                self.num_infected += 1;
            }
        } else {
            match self.advance_b() {
                InfectionState::Clean => {
                    self.dir = self.dir.turn_left();
                },
                InfectionState::Weakened => {
                    self.num_infected += 1;
                },
                InfectionState::Infected => {
                    self.dir = self.dir.turn_right();
                },
                InfectionState::Flagged => {
                    self.dir = self.dir.reverse();
                },
            }
        }

        let offset = self.dir.step_offset();
        self.pos = (self.pos.0 + offset.0, self.pos.1 + offset.1);

        Some(self.num_infected)
    }
}

fn count_infected(worm : WormProgress, iterations : usize) -> u32 {
    worm.take(iterations).last().unwrap()
}

fn count_infected_a(input : &str, iterations : usize) -> u32 {
    let worm = WormProgress::load(input, true);
    count_infected(worm, iterations)
}

fn count_infected_b(input : &str, iterations : usize) -> u32 {
    let worm = WormProgress::load(input, false);
    count_infected(worm, iterations)
}

fn solve_a(input : &str) -> u32 {
    count_infected_a(input, 10000)
}

fn solve_b(input : &str) -> u32 {
    count_infected_b(input, 10000000)
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
        assert_eq!(count_infected_a(&input, 1), 1);
        assert_eq!(count_infected_a(&input, 7), 5);
        assert_eq!(count_infected_a(&input, 70), 41);
        assert_eq!(solve_a(&input), 5587);
    }

    #[test]
    fn b_given() {
        let input =
r"..#
#..
...";
        assert_eq!(count_infected_b(&input, 1), 0);
        assert_eq!(count_infected_b(&input, 100), 26);
        assert_eq!(solve_b(&input), 2511944);
    }
}
