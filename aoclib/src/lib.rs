use std::io::prelude::*;

pub fn read_all_stdin() -> String {
    let mut contents = String::new();
    std::io::stdin().read_to_string(&mut contents).expect("failed to read input from stdin");
    contents.trim().to_string()
}
