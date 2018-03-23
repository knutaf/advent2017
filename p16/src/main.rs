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

struct RemovableRange {
    start : usize,
    end_opt : Option<usize>,
    generation : u32,
}

struct RangeHolder {
    ranges : HashMap<u64, RemovableRange>,
    valid_ranges : Vec<u64>,
    generation : u32,
}

struct Dance {
    moves : Vec<DanceMove>,
    multiplier : u64,
}

trait Performance : Iterator {
    fn positions(&self) -> String;
    fn rewind(&mut self);

    fn finish(&mut self) {
        //eprintln!("start: {}", self.positions());

        //let mut i = 0;
        while self.next().is_some() {
            //eprintln!("{}: {}", i, self.positions());
            //i += 1;
        }
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

    fn is_starting_positions(&self) -> bool {
        false
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

    position_for_dancer : [u8 ; NUM_DANCERS as usize],
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
        let moves = moves.split(',').map(DanceMove::from).filter(|step| {
            // Omit moves that do nothing
            match step {
                &DanceMove::Spin(a) => a != 0,
                &DanceMove::Exchange(a, b) => a != b,
                &DanceMove::Partner(a, b) => a != b,
            }
        }).collect::<Vec<DanceMove>>();

        let moves_len = moves.len();

        Dance {
            moves : moves,
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

    fn find_repeat_performance(&self, limit_times : u64) -> u64 {
        let mut performance = self.perform_int();

        let mut repeats_after_iterations = 1;
        performance.finish();

        while repeats_after_iterations < limit_times && !performance.is_starting_positions() {
            repeats_after_iterations += 1;
            performance.rewind();
            performance.finish();
        }

        repeats_after_iterations
    }

    fn finish_performance<P>(mut performance : P, num_times : u64) -> String
    where P : Performance {
        performance.finish();

        const OUTPUT_ITERATIONS : u64 = 100000;
        let mut iteration_counter = 0;
        let mut period_time = Instant::now();

        //eprintln!("poses after 0: {}", final_positions);
        for i in 1 .. num_times {
            iteration_counter += 1;
            if iteration_counter == OUTPUT_ITERATIONS {
                let elapsed = period_time.elapsed();
                let elapsed_ms = ((elapsed.as_secs() * 1000) as u32) + (elapsed.subsec_nanos() / 1000000);

                eprintln!("poses after {} of {}: {}. {} iterations per sec", i, num_times, performance.positions(), ((OUTPUT_ITERATIONS * 1000) as f32) / elapsed_ms as f32);

                iteration_counter = 0;
                period_time = Instant::now();
            }

            performance.rewind();
            performance.finish();
        }

        performance.positions()
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

    fn find_removable_ranges(&self, ranges : &mut RangeHolder) {
        ranges.start_new_generation();

        let performance = self.perform_int();

        for (i, dancers) in performance.enumerate() {
            if let Some(remrange) = ranges.ranges.get_mut(&dancers) {
                if remrange.generation == ranges.generation {
                    remrange.end_opt = Some(i);
                    ranges.valid_ranges.push(dancers);
                    //eprintln!("{}: {:016x} previously at {}. range: {}", i, dancers, remrange.start, i - remrange.start);
                    eprintln!("{}: {} previously at {}. range: {}", i, PerformanceInt::dancers_u64_to_string(&dancers), remrange.start, i - remrange.start);
                } else {
                    remrange.generation = ranges.generation;
                    remrange.start = i;
                    //eprintln!("{}: {:016x}", i, dancers);
                    //eprintln!("{}: {}", i, PerformanceInt::dancers_u64_to_string(&dancers));
                }
            } else {
                //eprintln!("{}: {:016x}", i, dancers);
                //eprintln!("{}: {}", i, PerformanceInt::dancers_u64_to_string(&dancers));
                ranges.ranges.insert(dancers, RemovableRange { start : i, end_opt : None, generation : ranges.generation });
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
    }

    fn collapse_removable_ranges(&mut self, ranges : &mut RangeHolder) -> bool {
        let mut made_change = false;
        self.find_removable_ranges(ranges);

        /*
        let mut removable_ranges_sorted = ranges.values().collect::<Vec<&(usize, Option<usize>)>>();
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

/*
  0      1      2      3      4      5
  A->B   B->C   C->D   D->C   C->E   E->D
A      B      C      D      C      E      D

  0      1      4      5
  A->B   B->C   C->E   E->D
A      B      C      E      D

  0      1      2      3      4      5
  A->B   B->C   C->D   D->C   C->E   E->D
A      B      C      D      C      E      D

C: start = 1,
   end = 3

D: start = 2
   end = 5

       0      1      2      3      4      5      6      7
       A->B   B->C   C->D   D->E   E->C   C->F   F->E   E->G
A      B      C      D      E      C      F      E      G
              |--------------------|
                            |--------------------|

       0      1      2      3      4      5      6      7
       A->B   B->C                        C->F   F->E   E->G
A      B      C                           F      E      G

       0      1      2      3      4      5      6      7
       A->B   B->C                               _->E   E->G
A      B      C                                  E      G
*/

        const MIN_RANGE_THRESHOLD_PERCENTAGE_HUNDREDTHS : usize = 50;
        let mut range_threshold_percentage_hundredths = 50000;

        loop {
            let mut changes_in_this_loop = 0;
            //eprintln!("trying range threshold {}: {} moves", range_threshold_percentage_hundredths, ((self.moves.len() * range_threshold_percentage_hundredths) / 100000));

            let removable_ranges_iter = ranges.valid_ranges.iter();
            for dancers in removable_ranges_iter {
                let remrange = ranges.ranges.get(&dancers).unwrap();
                if remrange.generation == ranges.generation {
                    if let Some(&end) = remrange.end_opt.as_ref() {
                        made_change = true;
                        if (end - (remrange.start + 1)) >= ((self.moves.len() * range_threshold_percentage_hundredths) / 100000) &&
                            self.moves[remrange.start + 1] != DanceMove::Spin(0) &&
                            self.moves[end] != DanceMove::Spin(0) {
                            changes_in_this_loop += 1;
                            for i in remrange.start+1 .. end+1 {
                                //eprintln!("{}: {} -> s0", i, self.moves[i]);
                                self.moves[i] = DanceMove::Spin(0);
                            }
                        } else {
                            //eprintln!("range {}-{} ({}) didn't work. start valid: {}, end valid: {}", end, remrange.start, (end - 1 - remrange.start), self.moves[remrange.start] != DanceMove::Spin(0), self.moves[end-1] != DanceMove::Spin(0));
                        }
                    }
                }
            }

            if changes_in_this_loop == 0 {
                if range_threshold_percentage_hundredths <= MIN_RANGE_THRESHOLD_PERCENTAGE_HUNDREDTHS {
                    break;
                } else {
                    range_threshold_percentage_hundredths /= 2;
                }
            } else {
                //eprintln!("removed {}", changes_in_this_loop);
            }
        }

        made_change
    }

    fn collapse_spins(&mut self) -> bool {
        /*
        0123456789abcdef
        x0/1
        1023456789abcdef
        s1
        f1023456789abcde

        0123456789abcdef
        s1
        f0123456789abcde
        x1/2
        f1023456789abcde

        s1
        x1/2
        s1
        x1/2

        s2
        x2/3
        x1/2
        */

        /*
               0123456789abcdef
        s1     f0123456789abcde
        x1/2   f1023456789abcde
        s1     ef1023456789abcd
        x1/2   e1f023456789abcd
        s1     de1f023456789abc
        x1/2   d1ef023456789abc
        s1     cd1ef023456789ab

               0123456789abcdef
        s4     cdef0123456789ab
        x4/5   cdef1023456789ab
        x3/4   cde1f023456789ab
        x2/3   cd1ef023456789ab
        */
        let mut total_spin : u8 = 0;
        let mut first_spin_index = 0;
        for (i, step) in self.moves.iter_mut().enumerate().rev() {
            match step {
                &mut DanceMove::Spin(spin) => {
                    total_spin = (((total_spin as u32) + spin) % (NUM_DANCERS as u32)) as u8;
                    *step = DanceMove::Spin(0);
                    first_spin_index = i;
                },
                &mut DanceMove::Exchange(a, b) => {
                    *step = DanceMove::Exchange((a + total_spin) % NUM_DANCERS, (b + total_spin) % NUM_DANCERS);
                },
                &mut DanceMove::Partner(a, b) => (),
            }
        }

        if total_spin != 0 {
            self.moves[first_spin_index] = DanceMove::Spin(total_spin as u32);
            true
        } else {
            false
        }
    }

    fn refine(&mut self, ranges : &mut RangeHolder) -> bool {
        if self.multiplier == 1 {
            self.collapse_spins();

            let pre_moves = self.moves.len();
            eprintln!("before: {} moves", self.moves.len());
            self.moves.retain(|step| {
                *step != DanceMove::Spin(0)
            });
            eprintln!("removed {}: {} moves left", pre_moves - self.moves.len(), self.moves.len());
            // eprintln!("{}", self);
        }

        //self.collapse_removable_ranges(ranges);

        let pre_moves = self.moves.len();
        eprintln!("before: {} moves", self.moves.len());
        self.moves.retain(|step| {
            *step != DanceMove::Spin(0)
        });
        eprintln!("removed {}: {} moves left", pre_moves - self.moves.len(), self.moves.len());
        // eprintln!("{}", self);

        self.moves.len() != pre_moves
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

impl RangeHolder {
    fn new(dance : &Dance) -> RangeHolder {
        RangeHolder {
            ranges : HashMap::with_capacity(5000000),
            valid_ranges : Vec::with_capacity(dance.moves.len()),
            generation : 0,
        }
    }

    fn start_new_generation(&mut self) {
        self.generation += 1;
        self.valid_ranges.truncate(0);
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
            position_for_dancer : [ 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, ],
            steps : &moves,
            position : 0,
        }
    }

    fn get_shift_for_position(position : u8) -> u32 {
        ((NUM_DANCERS - 1 - position) * (BITS_PER_DANCER as u8)) as u32
    }

    fn get_dancer_at_position(dancers : &u64, position : u8) -> u8 {
        ((dancers >> Self::get_shift_for_position(position)) & DANCER_MASK) as u8
    }

    fn get_position_of_dancer(&self, dancer : u8) -> u8 {
        self.position_for_dancer[dancer as usize]
    }

    fn set_dancers_at_positions(&mut self, position1 : u8, dancer1 : u8, position2 : u8, dancer2 : u8) {
        self.dancer_at_position = self.dancer_at_position &
            !(DANCER_MASK << Self::get_shift_for_position(position1)) &
            !(DANCER_MASK << Self::get_shift_for_position(position2)) |
            (((dancer1 as u64) & DANCER_MASK) << Self::get_shift_for_position(position1)) |
            (((dancer2 as u64) & DANCER_MASK) << Self::get_shift_for_position(position2));

        self.position_for_dancer[dancer1 as usize] = position1;
        self.position_for_dancer[dancer2 as usize] = position2;
    }

    fn recompute_positions_for_dancers(&mut self) {
        for position in 0 .. NUM_DANCERS {
           self.position_for_dancer[Self::get_dancer_at_position(&self.dancer_at_position, position) as usize] = position;
        }
    }

    fn dancers_u64_to_string(dancers : &u64) -> String {
        Self::dancers_to_string((0 .. NUM_DANCERS).map(|i| {
            Self::get_dancer_at_position(dancers, i)
        }))
    }
}

impl<'t> Performance for PerformanceInt<'t> {
    fn positions(&self) -> String {
        Self::dancers_u64_to_string(&self.dancer_at_position)
    }

    fn rewind(&mut self) {
        self.position = 0;
    }

    fn is_starting_positions(&self) -> bool {
        self.dancer_at_position == 0x0123456789abcdef
    }
}

impl<'t> Iterator for PerformanceInt<'t> {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.position < self.steps.len() {
            match self.steps[self.position] {
                DanceMove::Spin(count) => {
                    self.dancer_at_position = self.dancer_at_position.rotate_right(count * (BITS_PER_DANCER as u32));
                    self.recompute_positions_for_dancers();
                },
                DanceMove::Exchange(a, b) => {
                    let dancer_a = Self::get_dancer_at_position(&self.dancer_at_position, a);
                    let dancer_b = Self::get_dancer_at_position(&self.dancer_at_position, b);
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
    let mut ranges = RangeHolder::new(&dance);
    let before = dance.get_final_positions(NUM_DANCERS, 1);

    //dance.refine(&mut ranges);
    while dance.refine(&mut ranges) {}

    let after = dance.get_final_positions(NUM_DANCERS, 1);

    eprintln!("compare: {} and {}", before, after);
    before
}

fn solve_b(input : &str) -> String {
    let mut dance = Dance::from(input);
    let mut ranges = RangeHolder::new(&dance);
    const NUM_TIMES_B : u64 = 1000000000;
    //const NUM_TIMES_B : u64 = 2;
    //const MAX_MULTIPLIER : u64 = NUM_TIMES_B;
    const MAX_DANCE_LENGTH : usize = 3000000;
    const MAX_MULTIPLIER : u64 = 1;
    //const MAX_DANCE_LENGTH : usize = 1;
    const REPEAT_PERFORMANCE_SEARCH_LIMIT : u64 = 100;

    while dance.multiplier < MAX_MULTIPLIER {
        while dance.refine(&mut ranges) {}
        //eprintln!("dance: {}", dance);

        if dance.moves.len() <= MAX_DANCE_LENGTH {
            dance.multiply();
        } else {
            break;
        }
    }

    if dance.moves.len() <= MAX_DANCE_LENGTH {
        while dance.refine(&mut ranges) {}
    }
    //eprintln!("dance: {}", dance);

    let num_times;
    let repeats_after_iterations = dance.find_repeat_performance(REPEAT_PERFORMANCE_SEARCH_LIMIT);
    if repeats_after_iterations < REPEAT_PERFORMANCE_SEARCH_LIMIT {
        num_times = NUM_TIMES_B % repeats_after_iterations;
    } else {
        num_times = NUM_TIMES_B;
    }

    eprintln!("need to do {} iterations", num_times);

    let answer = dance.get_final_positions_int(num_times / dance.multiplier);

    /*
    let mut dance = Dance::from(input);
    let oracle = dance.get_final_positions_int(NUM_TIMES_B);
    eprintln!("oracle: {}. matches: {}", oracle, oracle == answer);
    */

    answer
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
        let mut ranges = RangeHolder::new(&dance);

        const NUM_REFINES : u64 = 5;
        let mut num_times = 1u64;
        for _ in 0 .. NUM_REFINES {
            num_times *= REFINE_MULTIPLIER;
        }

        let expected_final_positions = dance.get_final_positions_int(num_times);

        for _ in 0 .. NUM_REFINES {
            while dance.refine(&mut ranges) {}
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

// abcdefghijklmnop

// bcdaefghijklmnop
// cdabefghijklmnop
// dabcefghijklmnop
// abcdefghijklmnop

// bcdaefghijklmnop
// cdabefghijklmnop
// dabcefghijklmnop
// abcdefghijklmnop

// bcdaefghijklmnop
// cdabefghijklmnop
