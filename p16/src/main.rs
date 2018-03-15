#![feature(nll)]
#![feature(universal_impl_trait)]

use std::fmt;
use std::time::Instant;
use std::collections::HashMap;
use std::cmp::Ordering;

#[macro_use] extern crate lazy_static;
extern crate regex;
use regex::Regex;

extern crate aoclib;
use aoclib::*;

const NUM_DANCERS : u8 = 16;
const BITS_PER_DANCER : u64 = 4;
const DANCER_MASK : u64 = (1 << BITS_PER_DANCER) - 1;
const REFINE_MULTIPLIER : u64 = 10;

#[derive(PartialEq, Clone)]
enum DanceMove {
    Spin(u32),
    Exchange(u8, u8),
    Partner(u8, u8),
}

struct Dance {
    moves : Vec<DanceMove>,
    multiplier : u64,
}

trait Performance : Iterator {
    fn positions(&self) -> String;
    fn rewind(&mut self);

    fn finish(&mut self) -> String {
        while self.next().is_some() {
        }
        self.positions()
    }

    fn dancers_to_string<'t>(dancers : impl Iterator<Item = u8>) -> String
    {
        // TODO: try with map and collect
        let mut result = String::new();
        for dancer in dancers {
            result.push(DanceMove::dancer_number_to_name(dancer));
        }
        result
    }
}

struct PerformanceString<'t> {
    dancers : Vec<u8>,
    steps : std::iter::Cycle<std::slice::Iter<'t, DanceMove>>,
    num_steps : usize,
    position : usize,
}

struct PerformanceInt<'t> {
    // packed array, where each slot is the dancer number at that position
    dancer_at_position : u64,
    steps : &'t Vec<DanceMove>,
    position : usize,
}

impl DanceMove {
    fn dancer_name_to_number(dancer : &str) -> u8 {
        dancer.bytes().nth(0).unwrap() - ('a' as u8)
    }

    fn dancer_number_to_name(dancer : u8) -> char {
        (('a' as u8) + dancer) as char
    }

    fn from(input : &str) -> DanceMove {
        lazy_static! {
            static ref RE_SPIN : regex::Regex = Regex::new(r"^s(\d+)$").expect("failed to compile regex");
            static ref RE_EXCHANGE : regex::Regex = Regex::new(r"^x(\d+)/(\d+)$").expect("failed to compile regex");
            static ref RE_PARTNER : regex::Regex = Regex::new(r"^p(\w+)/(\w+)$").expect("failed to compile regex");
        }

        if let Some(captures) = RE_SPIN.captures_iter(input).next() {
            DanceMove::Spin(captures.get(1).unwrap().as_str().parse::<u32>().unwrap())
        } else if let Some(captures) = RE_EXCHANGE.captures_iter(input).next() {
            DanceMove::Exchange(captures.get(1).unwrap().as_str().parse::<u8>().unwrap(), captures.get(2).unwrap().as_str().parse::<u8>().unwrap())
        } else if let Some(captures) = RE_PARTNER.captures_iter(input).next() {
            DanceMove::Partner(Self::dancer_name_to_number(captures.get(1).unwrap().as_str()), Self::dancer_name_to_number(captures.get(2).unwrap().as_str()))
        } else {
            panic!("invalid move {}", input);
        }
    }
}

impl fmt::Display for DanceMove {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &DanceMove::Spin(a) => write!(f, "s{}", a),
            &DanceMove::Exchange(a, b) => write!(f, "x{}/{}", a, b),
            &DanceMove::Partner(a, b) => write!(f, "p{}/{}", Self::dancer_number_to_name(a), Self::dancer_number_to_name(b)),
        }
    }
}

impl Dance {
    fn from(moves : &str) -> Dance {
        Dance {
            moves : moves.split(',').map(DanceMove::from).filter(|step| {
                // Omit moves that do nothing
                match step {
                    &DanceMove::Spin(a) => a != 0,
                    &DanceMove::Exchange(a, b) => a != b,
                    &DanceMove::Partner(a, b) => a != b,
                }
            }).collect(),
            multiplier : 1,
        }
    }

    fn perform(&self, num_dancers : u8) -> PerformanceString {
        PerformanceString::new(&self.moves, num_dancers)
    }

    fn perform_int(&self) -> PerformanceInt {
        PerformanceInt::new(&self.moves)
    }

    fn get_final_positions(&self, num_dancers : u8, num_times : u64) -> String {
        Self::finish_performance(self.perform(num_dancers), num_times)
    }

    fn get_final_positions_int(&self, num_times : u64) -> String {
        Self::finish_performance(self.perform_int(), num_times)
    }

    fn finish_performance<P>(mut performance : P, num_times : u64) -> String
    where P : Performance {
        let mut final_positions = performance.finish();

        const OUTPUT_ITERATIONS : u64 = 1;
        let mut iteration_counter = 0;
        let mut period_time = Instant::now();

        //eprintln!("poses after 0: {}", final_positions);
        for i in 1 .. num_times {
            iteration_counter += 1;
            if iteration_counter == OUTPUT_ITERATIONS {
                let elapsed = period_time.elapsed();
                let elapsed_ms = ((elapsed.as_secs() * 1000) as u32) + (elapsed.subsec_nanos() / 1000000);

                eprintln!("poses after {}: {}. {} iterations per sec", i, final_positions, ((OUTPUT_ITERATIONS * 1000) as f32) / elapsed_ms as f32);

                iteration_counter = 0;
                period_time = Instant::now();
            }

            performance.rewind();
            final_positions = performance.finish();
        }

        final_positions
    }

    fn find_removable_ranges(&self) -> HashMap<u64, (usize, Option<usize>)> {
        let mut history : HashMap<u64, (usize, Option<usize>)> = HashMap::with_capacity(self.moves.len());

        let performance = self.perform_int();

        for (i, dancers) in performance.enumerate() {
            if let Some(poses) = history.get_mut(&dancers) {
                poses.1 = Some(i);
                //eprintln!("{}: {:016x} previously at {}. range: {}", i, dancers, poses.0, i - poses.0);
            } else {
                //eprintln!("{}: {:016x}", i, dancers);
                history.insert(dancers, (i, None));
            }
        }

        /*
        let mut ranges : Vec<(usize, usize)> = history.values().filter_map(|&(start, end_opt)| {
            end_opt.map(|end| {
                (start, end)
            })
        }).collect();
        */

        /*
        ranges.sort_unstable_by(|&(start1, end1), &(start2, end2)| {
            if start1 == start2 {
                (end1 - start1).cmp(&(end2 - start2))
            } else {
                start1.cmp(&start2)
            }
        });

        ranges
        */

        history
    }

    fn multiply(&mut self) {
        eprintln!("before multiply: {} moves", self.moves.len());
        self.multiplier *= REFINE_MULTIPLIER;
        let old_moves = self.moves.clone();
        for _ in 0 .. (REFINE_MULTIPLIER - 1) {
            self.moves.extend(old_moves.iter().cloned());
        }
        eprintln!("after multiplier {}: {} moves", self.multiplier, self.moves.len());
        // eprintln!("{}", self);
    }

    fn refine(&mut self) -> bool {
        let removable_ranges = self.find_removable_ranges();
        /*
        let mut removable_ranges_sorted = removable_ranges.values().collect::<Vec<&(usize, Option<usize>)>>();
        removable_ranges_sorted.sort_unstable_by(|&&(start1, end_opt1), &&(start2, end_opt2)| {
            if start1 == start2 {
                if let Some(end1) = end_opt1 {
                    if let Some(end2) = end_opt2 {
                        (end1 - start1).cmp(&(end2 - start2))
                    } else {
                        Ordering::Less
                    }
                } else {
                    if end_opt2.is_some() {
                        Ordering::Greater
                    } else {
                        Ordering::Equal
                    }
                }
            } else {
                start1.cmp(&start2)
            }
        });

        let removable_ranges_iter = removable_ranges_sorted.iter();
        */

        let mut made_change = false;

/*
  1      2      3      4      5      6
  A->B   B->C   C->D   D->C   C->E   E->D
A      B      C      D      C      E      D

  1      2      5      6
  A->B   B->C   C->E   E->D
A      B      C      E      D

  1      2      3      4      5      6
  A->B   B->C   C->D   D->C   C->E   E->D
A      B      C      D      C      E      D
              |--------------------|

  1      2      3      4      5      6
  A->B   B->C                        E->D
A      B                                  D
*/

        let mut range_threshold_percentage = 50;

        loop {
            //eprintln!("trying range threshold {}: {} moves", range_threshold_percentage, ((self.moves.len() * range_threshold_percentage) / 100));

            let removable_ranges_iter = removable_ranges.values();
            for &(start, end_opt) in removable_ranges_iter {
                if let Some(&end) = end_opt.as_ref() {
                    made_change = true;

                    if (end - 1 - start) >= ((self.moves.len() * range_threshold_percentage) / 100) &&
                        self.moves[start] != DanceMove::Spin(0) &&
                        self.moves[end-1] != DanceMove::Spin(0) {
                        for i in start .. end {
                            //eprintln!("{}: {} -> s0", i, self.moves[i]);
                            self.moves[i] = DanceMove::Spin(0);
                        }
                    }
                }
            }

            if range_threshold_percentage == 0 {
                break;
            } else {
                range_threshold_percentage /= 2;
            }
        }

        let pre_moves = self.moves.len();
        eprintln!("before: {} moves", self.moves.len());
        self.moves.retain(|step| {
            *step != DanceMove::Spin(0)
        });
        eprintln!("removed {} in {} ranges: {} moves left", pre_moves - self.moves.len(), removable_ranges.len(), self.moves.len());
        // eprintln!("{}", self);

        made_change
    }
}

impl fmt::Display for Dance {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let ret = write!(f, "");
        for (i, step) in self.moves.iter().enumerate() {
            write!(f, "{}:    {}\n", i, step);
        }
        ret
    }
}

impl<'t> PerformanceString<'t> {
    fn new(moves : &'t Vec<DanceMove>, num_dancers : u8) -> PerformanceString<'t> {
        let moves_iter = moves.iter();
        let num_moves = moves.len();

        PerformanceString {
            dancers : (0u8 .. num_dancers).collect(),
            steps : moves_iter.cycle(),
            num_steps : num_moves,
            position : 0,
        }
    }
}

impl<'t> Performance for PerformanceString<'t> {
    fn positions(&self) -> String {
        Self::dancers_to_string(self.dancers.iter().map(|x| *x))
    }

    fn rewind(&mut self) {
        self.position = 0;
    }
}

impl<'t> Iterator for PerformanceString<'t> {
    type Item = ();

    fn next(&mut self) -> Option<Self::Item> {
        if self.position < self.num_steps {
            self.position += 1;

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
                            if poses.0.is_none() && *item == a {
                                //eprintln!("found a ({}) at pos {}. item is {}", a, i, item);
                                poses = (Some(i), poses.1);
                            }

                            if poses.1.is_none() && *item == b {
                                //eprintln!("found b ({}) at pos {}. item is {}", b, i, item);
                                poses = (poses.0, Some(i));
                            }

                            poses
                        });

                        self.dancers.swap(a_pos.unwrap(), b_pos.unwrap());
                    },
                };
            })
        } else {
            None
        }
    }
}

impl<'t> PerformanceInt<'t> {
    fn new(moves : &'t Vec<DanceMove>) -> PerformanceInt<'t> {
        let dancers_init : u64 =
            15 << (0 * BITS_PER_DANCER) |
            14 << (1 * BITS_PER_DANCER) |
            13 << (2 * BITS_PER_DANCER) |
            12 << (3 * BITS_PER_DANCER) |
            11 << (4 * BITS_PER_DANCER) |
            10 << (5 * BITS_PER_DANCER) |
            9 << (6 * BITS_PER_DANCER) |
            8 << (7 * BITS_PER_DANCER) |
            7 << (8 * BITS_PER_DANCER) |
            6 << (9 * BITS_PER_DANCER) |
            5 << (10 * BITS_PER_DANCER) |
            4 << (11 * BITS_PER_DANCER) |
            3 << (12 * BITS_PER_DANCER) |
            2 << (13 * BITS_PER_DANCER) |
            1 << (14 * BITS_PER_DANCER) |
            0 << (15 * BITS_PER_DANCER);

        PerformanceInt {
            dancer_at_position : dancers_init,
            steps : &moves,
            position : 0,
        }
    }

    fn get_shift_for_position(position : u8) -> u32 {
        ((NUM_DANCERS - 1 - position) * (BITS_PER_DANCER as u8)) as u32
    }

    fn get_dancer_at_position(&self, position : u8) -> u8 {
        ((self.dancer_at_position >> Self::get_shift_for_position(position)) & DANCER_MASK) as u8
    }

    fn get_position_of_dancer(&self, dancer : u8) -> u8 {
        let mut position = 0;
        while position < NUM_DANCERS {
            if self.get_dancer_at_position(position) == dancer {
                return position;
            }

            position += 1;
        }

        panic!("couldn't find position for dancer {}", dancer);
    }

    fn set_dancers_at_positions(&mut self, position1 : u8, dancer1 : u8, position2 : u8, dancer2 : u8) {
        self.dancer_at_position = self.dancer_at_position &
            !(DANCER_MASK << Self::get_shift_for_position(position1)) &
            !(DANCER_MASK << Self::get_shift_for_position(position2)) |
            (((dancer1 as u64) & DANCER_MASK) << Self::get_shift_for_position(position1)) |
            (((dancer2 as u64) & DANCER_MASK) << Self::get_shift_for_position(position2));
    }
}

impl<'t> Performance for PerformanceInt<'t> {
    fn positions(&self) -> String {
        Self::dancers_to_string((0 .. NUM_DANCERS).map(|i| {
            self.get_dancer_at_position(i)
        }))
    }

    fn rewind(&mut self) {
        self.position = 0;
    }
}

impl<'t> Iterator for PerformanceInt<'t> {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.position < self.steps.len() {
            match self.steps[self.position] {
                DanceMove::Spin(count) => {
                    self.dancer_at_position = self.dancer_at_position.rotate_right(count * (BITS_PER_DANCER as u32));
                },
                DanceMove::Exchange(a, b) => {
                    let dancer_a = self.get_dancer_at_position(a);
                    let dancer_b = self.get_dancer_at_position(b);
                    self.set_dancers_at_positions(a, dancer_b, b, dancer_a);
                },
                DanceMove::Partner(a, b) => {
                    let a_pos = self.get_position_of_dancer(a);
                    let b_pos = self.get_position_of_dancer(b);
                    self.set_dancers_at_positions(a_pos, b, b_pos, a);
                },
            };

            self.position += 1;
            Some(self.dancer_at_position)
        } else {
            None
        }
    }
}

fn solve_a(input : &str) -> String {
    let mut dance = Dance::from(input);
    let before = dance.get_final_positions(NUM_DANCERS, 1);

    dance.refine();

    let after = dance.get_final_positions(NUM_DANCERS, 1);

    eprintln!("compare: {} and {}", before, after);
    before
}

fn solve_b(input : &str) -> String {
    let mut dance = Dance::from(input);
    //const NUM_TIMES_B : u64 = 1000000000;
    const NUM_TIMES_B : u64 = 1000000000;
    const MAX_MULTIPLIER : u64 = 10;

    while dance.multiplier < MAX_MULTIPLIER {
        while dance.refine() {}
        //eprintln!("dance: {}", dance);
        dance.multiply();
    }

    while dance.refine() {}
    //eprintln!("dance: {}", dance);

    dance.get_final_positions_int(NUM_TIMES_B / MAX_MULTIPLIER)
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

    fn test_dance_int_repeat(moves : &str, num_times : u64, expected_final_positions : &str) {
        let dance = Dance::from(moves);
        assert_eq!(dance.get_final_positions_int(num_times), expected_final_positions);
    }

    fn test_dance_repeat(num_dancers : u8, moves : &str, num_times : u64, expected_final_positions : &str) {
        let dance = Dance::from(moves);
        assert_eq!(dance.get_final_positions(num_dancers, num_times), expected_final_positions);
    }

    fn test_dance(num_dancers : u8, moves : &str, expected_final_position : &str) {
        test_dance_repeat(num_dancers, moves, 1, expected_final_position)
    }

    #[test]
    fn spin() {
        test_dance(5, "s1", "eabcd");
        test_dance(5, "s2", "deabc");
        test_dance(5, "s5", "abcde");
        test_dance(5, "s10", "abcde");
    }

    #[test]
    fn spin_int() {
        test_dance_int_repeat("s1", 1, "pabcdefghijklmno");
        test_dance_int_repeat("s2", 1, "opabcdefghijklmn");
        test_dance_int_repeat("s16", 1, "abcdefghijklmnop");
        test_dance_int_repeat("s17", 1, "pabcdefghijklmno");
        test_dance_int_repeat("s32", 1, "abcdefghijklmnop");
    }

    #[test]
    fn exchange() {
        test_dance(5, "x0/1", "bacde");
        test_dance(5, "x0/0", "abcde");
        test_dance(5, "x0/4", "ebcda");
    }

    #[test]
    fn exchange_int() {
        test_dance_int_repeat("x0/1", 1, "bacdefghijklmnop");
        test_dance_int_repeat("x0/0", 1, "abcdefghijklmnop");
        test_dance_int_repeat("x0/15", 1, "pbcdefghijklmnoa");
    }

    #[test]
    fn partner() {
        test_dance(5, "pa/b", "bacde");
        test_dance(5, "pa/e", "ebcda");
        test_dance(5, "pa/a", "abcde");
    }

    #[test]
    fn partner_int() {
        test_dance_int_repeat("pa/b", 1, "bacdefghijklmnop");
        test_dance_int_repeat("pa/a", 1, "abcdefghijklmnop");
        test_dance_int_repeat("pa/p", 1, "pbcdefghijklmnoa");
    }

    #[test]
    fn simple_int_repeat() {
        test_dance_int_repeat("s1,x0/1,pa/b", 2, "aopbcdefghijklmn");
        test_dance_int_repeat("s1,x0/1,pa/b", 3, "bnopacdefghijklm");
    }

    #[test]
    fn refine_1() {
        let mut dance = Dance::from("s2,s15,s1,s1");

        const NUM_REFINES : u64 = 5;
        let mut num_times = 1u64;
        for _ in 0 .. NUM_REFINES {
            num_times *= REFINE_MULTIPLIER;
        }

        let expected_final_positions = dance.get_final_positions_int(num_times);

        for _ in 0 .. NUM_REFINES {
            while dance.refine() {}
            dance.multiply();
        }

        assert_eq!(dance.get_final_positions_int(1), expected_final_positions);
    }

    #[test]
    fn a_given() {
        test_dance(5, "s1,x3/4,pe/b", "baedc");
    }

    #[test]
    fn b_given() {
        test_dance_repeat(5, "s1,x3/4,pe/b", 2, "ceadb");
    }
}
