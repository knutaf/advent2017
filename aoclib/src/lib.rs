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
