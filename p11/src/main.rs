#![feature(nll)]

use std::cmp::Ordering;

extern crate aoclib;
use aoclib::*;

struct HexMover<'t> {
    pos : (i32, i32),
    steps : Vec<&'t str>,
    index : usize,
    at_end : bool,
}

impl<'t> HexMover<'t> {
    fn new(steps : &'t str) -> HexMover<'t> {
        let steps = steps.split(',').collect();

        HexMover {
            pos : (0, 0),
            steps : steps,
            index : 0,
            at_end : false,
        }
    }

    fn take_step(pos : &(i32, i32), step : &str) -> (i32, i32) {
        match step {
            "n" => (pos.0, pos.1 + 2),
            "s" => (pos.0, pos.1 - 2),
            "ne" => (pos.0 + 2, pos.1 + 1),
            "se" => (pos.0 + 2, pos.1 - 1),
            "nw" => (pos.0 - 2, pos.1 + 1),
            "sw" => (pos.0 - 2, pos.1 - 1),
            _ => panic!("invalid step {}", step),
        }
    }
}

impl<'t> Iterator for HexMover<'t> {
    type Item = (i32, i32);

    fn next(&mut self) -> Option<Self::Item> {
        if !self.at_end {
            if self.index < self.steps.len() {
                self.pos = HexMover::take_step(&self.pos, self.steps[self.index]);
                self.index += 1;
            } else {
                self.at_end = true;
            }

            Some(self.pos)
        } else {
            None
        }
    }
}

struct HexSeeker {
    pos : (i32, i32),
    target : (i32, i32),
}

impl HexSeeker {
    fn new(target : &(i32, i32)) -> HexSeeker {
        HexSeeker {
            pos : (0, 0),
            target : *target,
        }
    }
}

impl Iterator for HexSeeker {
    type Item = &'static str;

    fn next(&mut self) -> Option<Self::Item> {
        let ew = self.pos.0.cmp(&self.target.0);
        let ns = self.pos.1.cmp(&self.target.1);

        let next_step = match ew {
            Ordering::Equal => {
                match ns {
                    Ordering::Equal => None,
                    Ordering::Less => Some("n"),
                    Ordering::Greater => Some("s"),
                }
            },
            Ordering::Less => {
                match ns {
                    Ordering::Equal => Some("ne"),
                    Ordering::Less => Some("ne"),
                    Ordering::Greater => Some("se"),
                }
            },
            Ordering::Greater => {
                match ns {
                    Ordering::Equal => Some("nw"),
                    Ordering::Less => Some("nw"),
                    Ordering::Greater => Some("sw"),
                }
            },
        };

        if let Some(step) = next_step.as_ref() {
            self.pos = HexMover::take_step(&self.pos, step);
        }

        next_step
    }
}

fn solve_a(input : &str) -> u32 {
    let mover = HexMover::new(input);
    let target = mover.last().unwrap();

    let seeker = HexSeeker::new(&target);
    seeker.count() as u32
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

    fn test_coords(input : &str, final_pos : (i32, i32)) {
        let mover = HexMover::new(input);
        assert_eq!(mover.last().unwrap(), final_pos);
    }

    fn test_path(input : &str, expected_path : Vec<&'static str>) {
        let mover = HexMover::new(input);
        let target = mover.last().unwrap();

        let seeker = HexSeeker::new(&target);
        let mut actual_path : Vec<&str> = seeker.collect();
        actual_path.sort();
        let mut expected_path = expected_path.clone();
        expected_path.sort();

        assert_eq!(actual_path, expected_path);
    }

    #[test]
    fn move_zero() {
        test_coords("ne,ne,sw,sw", (0, 0));
        test_coords("nw,nw,se,se", (0, 0));
    }

    #[test]
    fn move_given() {
        test_coords("ne,ne,ne", (6, 3));
        test_coords("ne,ne,s,s", (4, -2));
        test_coords("se,sw,se,sw,sw", (-2, -5));
    }

    #[test]
    fn path_1() {
        test_path("n,se", vec!["ne"]);
        test_path("se,n", vec!["ne"]);

        test_path("n,sw", vec!["nw"]);
        test_path("sw,n", vec!["nw"]);

        test_path("s,ne", vec!["se"]);
        test_path("ne,s", vec!["se"]);

        test_path("s,nw", vec!["sw"]);
        test_path("nw,s", vec!["sw"]);
    }

    #[test]
    fn path_given() {
        test_path("ne,ne,ne", vec!["ne", "ne", "ne"]);
        test_path("ne,ne,sw,sw", vec![]);
        test_path("ne,ne,s,s", vec!["se", "se"]);
        test_path("se,sw,se,sw,sw", vec!["s", "s", "sw"]);
    }

    #[test]
    fn b_1() {
        let input = "blah";
        assert_eq!(solve_b(&input), 0);
    }
}
