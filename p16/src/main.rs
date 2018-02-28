#![feature(nll)]

use std::fmt;

#[macro_use] extern crate lazy_static;
extern crate regex;
use regex::Regex;

extern crate aoclib;
use aoclib::*;

const PART_A_NUM_DANCERS : u8 = 16;

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
    steps : std::slice::Iter<'t, DanceMove<'t>>,
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
    fn from(moves : &'t str) -> Dance<'t> {
        Dance {
            moves : moves.split(',').map(DanceMove::from).collect(),
        }
    }

    fn perform<'a>(&'a self, num_dancers : u8) -> Performance<'a> {
        Performance {
            dancers : (0 .. num_dancers).map(|i| {
                String::from(((('a' as u8) + i) as char).to_string())
            }).collect(),
            steps : self.moves.iter(),
        }
    }
}

impl<'t> Performance<'t> {
    fn positions(&self) -> String {
        let mut result = String::new();
        for dancer in self.dancers.iter() {
            result.push_str(dancer.as_str());
        }
        result
    }
}

impl<'t> Iterator for Performance<'t> {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        self.steps.next().map(|step| {
            match step {
                &DanceMove::Spin(count) => {
                    for _ in 0 .. count {
                        let end = self.dancers.pop().unwrap();
                        self.dancers.insert(0, end);
                    }
                },
                &DanceMove::Exchange(a, b) => {
                    self.dancers.swap(a as usize, b as usize);
                },
                &DanceMove::Partner(a, b) => {
                    let (a_pos, b_pos) = self.dancers.iter().enumerate().fold((None, None), |mut poses : (Option<usize>, Option<usize>), (i, item)| {
                        if poses.0.is_none() && item == a {
                            //eprintln!("found a ({}) at pos {}. item is {}", a, i, item);
                            poses = (Some(i), poses.1);
                        }

                        if poses.1.is_none() && item == b {
                            //eprintln!("found b ({}) at pos {}. item is {}", b, i, item);
                            poses = (poses.0, Some(i));
                        }

                        poses
                    });

                    self.dancers.swap(a_pos.unwrap(), b_pos.unwrap());
                },
            };

            self.positions()
        })
    }
}

fn solve_a(input : &str) -> String {
    Dance::from(input).perform(PART_A_NUM_DANCERS).last().unwrap()
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

    fn test_dance(num_dancers : u8, moves : &str, expected_final_position : &str) {
        let dance = Dance::from(moves);
        assert_eq!(dance.perform(num_dancers).last().unwrap(), expected_final_position);
    }

    #[test]
    fn spin() {
        test_dance(5, "s1", "eabcd");
        test_dance(5, "s2", "deabc");
        test_dance(5, "s5", "abcde");
        test_dance(5, "s10", "abcde");
    }

    #[test]
    fn exchange() {
        test_dance(5, "x0/1", "bacde");
        test_dance(5, "x0/0", "abcde");
        test_dance(5, "x0/4", "ebcda");
    }

    #[test]
    fn partner() {
        test_dance(5, "pa/b", "bacde");
        test_dance(5, "pa/e", "ebcda");
        test_dance(5, "pa/a", "abcde");
    }

    #[test]
    fn a_given() {
        test_dance(5, "s1,x3/4,pe/b", "baedc");
    }

    #[test]
    fn b_1() {
        let input = "blah";
        assert_eq!(solve_b(&input), 0);
    }
}
