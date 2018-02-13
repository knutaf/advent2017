#![feature(nll)]

use std::collections::HashSet;

#[macro_use] extern crate lazy_static;
extern crate regex;
use regex::Regex;

extern crate aoclib;
use aoclib::*;

// Stores just the raw parsed data from the input, not directly linked to
// supported programs.
#[derive(PartialEq, Debug)]
struct ProgInfo {
    name : u32,
    link_names : Vec<u32>,
}

struct ProgDb {
    db : Vec<ProgInfo>,
}

impl ProgDb {
    fn new() -> ProgDb {
        ProgDb {
            db : vec![],
        }
    }

    fn from_input(input : &str) -> ProgDb {
        let mut pdb = ProgDb::new();

        for line in input.lines() {
            pdb.db.push(parse_prog(line))
        }

        pdb
    }

    fn find_all_connected(&self, prog : u32) -> HashSet<u32> {
        let mut connected = HashSet::new();

        fn visit(this : &ProgDb, connected : &mut HashSet<u32>, prog : &ProgInfo) {
            if connected.insert(prog.name) {
                for link_name in prog.link_names.iter() {
                    visit(this, connected, &this.db[*link_name as usize]);
                }
            }
        }

        visit(self, &mut connected, &self.db[prog as usize]);

        connected
    }
}

fn parse_prog(line : &str) -> ProgInfo {
    lazy_static! {
        static ref RE_PROG_INFO : regex::Regex = Regex::new(r"^(\d+) <->").expect("failed to compile regex");
        static ref RE_PROG_LINKS : regex::Regex = Regex::new(r",? (\d+)").expect("failed to compile regex");
    }

    let prog_info_captures = RE_PROG_INFO.captures_iter(line).nth(0).unwrap();

    let link_captures_iter = RE_PROG_LINKS.captures_iter(line);

    ProgInfo {
        name : prog_info_captures.get(1).unwrap().as_str().parse::<u32>().unwrap(),
        link_names : link_captures_iter.map(|caps| {
            caps.get(1).unwrap().as_str().parse::<u32>().unwrap()
        }).collect(),
    }
}

fn find_connected_to(input : &str, prog : u32) -> HashSet<u32> {
    let pdb = ProgDb::from_input(input);
    let connected = pdb.find_all_connected(prog);
    eprintln!("connected: {:?}", connected);
    connected
}

fn solve_a(input : &str) -> u32 {
    find_connected_to(input, 0).len() as u32
}

fn solve_b(input : &str) -> u32 {
    let pdb = ProgDb::from_input(input);

    let mut all_sets : Vec<HashSet<u32>> = vec![];

    for prog in pdb.db.iter() {
        if !all_sets.iter().any(|set| {
            set.contains(&prog.name)
        }) {
            all_sets.push(pdb.find_all_connected(prog.name))
        }
    }

    all_sets.len() as u32
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

    fn test_connected_to(input : &str, prog : u32, expected_connected : &Vec<u32>) {
        use std::iter::FromIterator;
        let connected = find_connected_to(input, prog);
        let expected_connected = HashSet::from_iter(expected_connected.iter().map(|n| *n));
        assert_eq!(connected, expected_connected);
    }

    #[test]
    fn a_given() {
        let input =
r"0 <-> 2
1 <-> 1
2 <-> 0, 3, 4
3 <-> 2, 4
4 <-> 2, 3, 6
5 <-> 6
6 <-> 4, 5";

        test_connected_to(input, 0, &vec![0, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn a_alone() {
        let input =
r"0 <-> 0
1 <-> 1
2 <-> 3, 4
3 <-> 2, 4
4 <-> 2, 3, 6
5 <-> 6
6 <-> 4, 5";

        test_connected_to(input, 0, &vec![0]);
    }

    #[test]
    fn a_ring() {
        let input =
r"0 <-> 1, 5
1 <-> 0, 2
2 <-> 1, 3
3 <-> 2, 4
4 <-> 3, 5
5 <-> 0, 4
6 <-> 6";

        test_connected_to(input, 0, &vec![0, 1, 2, 3, 4, 5]);
    }

    #[test]
    fn b_given() {
        let input =
r"0 <-> 2
1 <-> 1
2 <-> 0, 3, 4
3 <-> 2, 4
4 <-> 2, 3, 6
5 <-> 6
6 <-> 4, 5";
        assert_eq!(solve_b(&input), 2);
    }

    #[test]
    fn b_singles() {
        let input =
r"0 <-> 0
1 <-> 1
2 <-> 2";
        assert_eq!(solve_b(&input), 3);
    }

    #[test]
    fn b_pairs() {
        let input =
r"0 <-> 1
1 <-> 0
2 <-> 3
3 <-> 2
4 <-> 5
5 <-> 4";
        assert_eq!(solve_b(&input), 3);
    }
}
