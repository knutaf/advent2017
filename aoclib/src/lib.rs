#![feature(nll)]

use std::io::prelude::*;
use std::env;

pub mod list;
pub mod knot_hash;
pub mod bit_iterator;
pub mod grid;
pub mod onoffpixel;
pub mod direction;

pub fn read_all_stdin() -> String {
    let mut contents = String::new();
    std::io::stdin().read_to_string(&mut contents).expect("failed to read input from stdin");
    contents.trim().to_string()
}

pub fn read_all_stdin_notrim() -> String {
    let mut contents = String::new();
    std::io::stdin().read_to_string(&mut contents).expect("failed to read input from stdin");
    contents.to_string()
}

pub fn should_solve_puzzle_a() -> bool {
    env::args().len() < 2
}

pub fn parse_nums<'t, T>(string : &'t str) -> impl Iterator<Item = T> + 't
    where T: std::str::FromStr + std::fmt::Debug {
    string.split_whitespace().map(|num_str| {
        num_str.parse::<T>().unwrap_or_else(|_| {
            panic!("failed to parse num");
        })
    })
}

pub fn position_eq<T>(mut iter : impl Iterator<Item = T>, item : &T) -> Option<usize>
    where T : PartialEq {
    iter.position(|x| { x == *item })
}

pub fn any_eq<T>(iter : impl Iterator<Item = T>, item : &T) -> bool
    where T : PartialEq {
    position_eq(iter, item).is_some()
}

pub fn reverse_circular_vec_segment<T>(v : &mut Vec<T>, start_index : usize, length : usize) {
    if length > 0 {
        let mut start_index = start_index;

        let mut end_index = start_index + length - 1;
        while end_index >= v.len() {
            end_index -= v.len();
        }

        for _ in 0 .. (length / 2) {
            v.swap(start_index, end_index);

            start_index =
                if start_index == v.len() - 1 {
                    0
                } else {
                    start_index + 1
                };

            end_index =
                if end_index == 0 {
                    v.len() - 1
                } else {
                    end_index - 1
                };
        }
    }
}

pub fn numbers_to_hex_string<T>(iter : impl Iterator<Item = T>) -> String
where T : std::fmt::LowerHex {
    let mut result = String::new();
    iter.fold((), |_, n| {
        result.push_str(format!("{0:01$x}", n, std::mem::size_of::<T>() * 2).as_str());
    });
    result
}

pub fn consume_iterator<T>(iter : &mut impl Iterator<Item = T>) {
    while let Some(_) = iter.next() { }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn reverse_segment_zero() {
        let mut v = vec![0, 1, 2, 3, 4, 5];
        reverse_circular_vec_segment(&mut v, 0, 0);
        assert_eq!(v, vec![0, 1, 2, 3, 4, 5]);
    }

    #[test]
    fn reverse_segment_one() {
        let mut v = vec![0, 1, 2, 3, 4, 5];
        reverse_circular_vec_segment(&mut v, 0, 1);
        assert_eq!(v, vec![0, 1, 2, 3, 4, 5]);
    }

    #[test]
    fn reverse_partial_segment() {
        let mut v = vec![0, 1, 2, 3, 4, 5];
        reverse_circular_vec_segment(&mut v, 1, 3);
        assert_eq!(v, vec![0, 3, 2, 1, 4, 5]);
    }

    #[test]
    fn reverse_partial_segment_wrap() {
        let mut v = vec![0, 1, 2, 3, 4, 5];
        reverse_circular_vec_segment(&mut v, 4, 3);
        assert_eq!(v, vec![4, 1, 2, 3, 0, 5]);
    }

    #[test]
    fn reverse_whole_segment() {
        let mut v = vec![0, 1, 2, 3, 4, 5];
        reverse_circular_vec_segment(&mut v, 0, 6);
        assert_eq!(v, vec![5, 4, 3, 2, 1, 0]);
    }

    #[test]
    fn reverse_whole_segment_wrap() {
        let mut v = vec![0, 1, 2, 3, 4, 5];
        reverse_circular_vec_segment(&mut v, 3, 6);
        assert_eq!(v, vec![5, 4, 3, 2, 1, 0]);
    }

    #[test]
    fn format_hex_u8() {
        assert_eq!(numbers_to_hex_string(0x0u8 .. 0x10u8), "000102030405060708090a0b0c0d0e0f");
    }

    #[test]
    fn format_hex_u32() {
        assert_eq!(numbers_to_hex_string(0x0u32 .. 0x2u32), "0000000000000001");
    }

    #[test]
    fn test_consume_iterator() {
        let mut iter = 1 .. 5;
        consume_iterator(&mut iter);
        assert_eq!(iter.next(), None);
    }
}
