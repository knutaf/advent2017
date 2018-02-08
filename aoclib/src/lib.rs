#![feature(nll)]
#![feature(conservative_impl_trait)]
#![feature(universal_impl_trait)]

use std::io::prelude::*;
use std::env;
pub mod list;

pub fn read_all_stdin() -> String {
    let mut contents = String::new();
    std::io::stdin().read_to_string(&mut contents).expect("failed to read input from stdin");
    contents.trim().to_string()
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

pub fn position_eq<T>(mut iter : impl Iterator<Item = T>, item : T) -> Option<usize>
    where T : PartialEq {
    iter.position(|x| { x == item })
}

pub fn any_eq<T>(iter : impl Iterator<Item = T>, item : T) -> bool
    where T : PartialEq {
    position_eq(iter, item).is_some()
}

pub fn reverse_circular_vec_segment<T>(v : &mut Vec<T>, start_index : usize, length : usize) {
    if length > 0 {
        let mut start_index = start_index;

        let mut end_index = start_index + length - 1;
        if end_index >= v.len() {
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
}
