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

struct DiskRowIterator<'t> {
    next_byte : std::slice::Iter<'t, u8>,
    next_bits : std::iter::Rev<aoclib::bit_iterator::BitIterator>,
}

fn bit_iterator_for_byte(byte : u8) -> std::iter::Rev<aoclib::bit_iterator::BitIterator> {
    aoclib::bit_iterator::BitIterator::new(byte).rev()
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
            let ret = Some(DiskRow::new(aoclib::knot_hash::knot_hash(&format!("{}-{}", self.seed, self.row_number))));

            self.row_number += 1;

            ret
        } else {
            None
        }
    }
}

impl DiskRow {
    fn new(bytes : Vec<u8>) -> DiskRow {
        DiskRow {
            row : bytes,
        }
    }

    fn iter_bits<'t>(&'t self) -> DiskRowIterator<'t> {
        let mut next_bytes = self.row.iter();
        let next_bits = bit_iterator_for_byte(*next_bytes.next().unwrap());
        DiskRowIterator {
            next_byte : next_bytes,
            next_bits : next_bits,
        }
    }
}

impl<'t> Iterator for DiskRowIterator<'t> {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        let bit = self.next_bits.next();
        if bit.is_some() {
            bit
        } else {
            if let Some(next_byte) = self.next_byte.next() {
                self.next_bits = bit_iterator_for_byte(*next_byte);
                self.next_bits.next()
            } else {
                None
            }
        }
    }
}

fn solve_a(input : &str) -> u32 {
    let rows = DiskRows::new(input, 128);
    rows.fold(0u32, |bits_used, row| {
        bits_used + row.iter_bits().map(|b| b as u32).sum::<u32>()
    })
}

fn solve_b(input : &str) -> u32 {
    let mut grid = aoclib::grid::Grid::<u32>::new();
    let rows = DiskRows::new(input, 128);

    for row in rows {
        grid.add_row(row.iter_bits().map(|v| v as u32).collect());
    }

    fn spread(grid : &mut aoclib::grid::Grid<u32>, x : usize, y : usize, old : u32, new : u32) {
        if *grid.get(x, y).unwrap() == old {
            *grid.get_mut(x, y).unwrap() = new;

            if y > 0 {
                spread(grid, x + 0, y - 1, old, new);
            }

            if x < grid.size_x() - 1 {
                spread(grid, x + 1, y + 0, old, new);
            }

            if y < grid.size_y() - 1 {
                spread(grid, x + 0, y + 1, old, new);
            }

            if x > 0 {
                spread(grid, x - 1, y + 0, old, new);
            }
        }
    };

    let mut num_islands = 1;
    while let Some(((found_x, found_y), _)) = grid.enumerate().find(|&(_, v)| *v == 1) {
        num_islands += 1;
        spread(&mut grid, found_x, found_y, 1, num_islands);
    }

    num_islands - 1
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
        assert_eq!(solve_a("flqrgnkx"), 8108);
    }

    #[test]
    fn b_1() {
        assert_eq!(solve_b("flqrgnkx"), 1242);
    }
}
