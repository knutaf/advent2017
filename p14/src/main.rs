#![feature(nll)]

extern crate aoclib;
use aoclib::*;

struct DiskRows<'t> {
    seed : &'t str,
    max_rows : u32,
    row_number : u32,
}

struct DiskRow {
    row : Vec<u8>,
}

impl<'t> DiskRows<'t> {
    fn new(seed : &'t str, max_rows : u32) -> DiskRows<'t> {
        DiskRows {
            seed : seed,
            max_rows : max_rows,
            row_number : 0,
        }
    }
}

impl<'t> Iterator for DiskRows<'t> {
    type Item = DiskRow;

    fn next(&mut self) -> Option<Self::Item> {
        if self.row_number < self.max_rows {
            let ret = Some(DiskRow {
                row : aoclib::knot_hash::knot_hash(&format!("{}-{}", self.seed, self.row_number)),
            });

            self.row_number += 1;

            ret
        } else {
            None
        }
    }
}

fn solve_a(input : &str) -> u32 {
    let rows = DiskRows::new(input, 128);
    rows.fold(0u32, |bits_used, row| {
        row.row.iter().fold(bits_used, |bits_used, byte| {
            bits_used + (aoclib::bit_iterator::BitIterator::new(*byte).rev().sum::<u8>() as u32)
        })
    })
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
        let input = "blah";
        assert_eq!(solve_a("flqrgnkx"), 8108);
    }

    #[test]
    fn b_1() {
        let input = "blah";
        assert_eq!(solve_b(&input), 0);
    }
}
