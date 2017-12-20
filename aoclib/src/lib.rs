#![feature(conservative_impl_trait)]

use std::io::prelude::*;
use std::env;

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
