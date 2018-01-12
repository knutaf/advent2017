extern crate aoclib;
use aoclib::*;

#[macro_use] extern crate lazy_static;
extern crate regex;
use regex::Regex;

use std::collections::HashMap;

#[derive(PartialEq, Debug)]
struct ProgInfo<'t> {
    name : &'t str,
    weight : u32,
    child_names : Vec<&'t str>,
}

struct Prog<'t> {
    name : &'t str,
    weight : u32,
    parent : Option<Box<Prog<'t>>>,
    children : Vec<Box<Prog<'t>>>,
}

struct ProgDb<'t> {
    db : HashMap<&'t str, Prog<'t>>,
}

impl<'t> ProgDb<'t> {
    fn new() -> ProgDb<'t> {
        ProgDb {
            db : HashMap::<&'t str, Prog<'t>>::new(),
        }
    }

    fn load_prog(&mut self, prog_info : ProgInfo<'t>) {
        let mut new_prog = match self.db.get_mut(prog_info.name) {
            None -> Prog {
                name : prog_info.name,
                weight : prog_info.weight,
                parent : None,
                children : vec![],
            },
            Some(prog) -> prog
        }

        for child in prog_info.child_names {
            match self.db.get(child) {
                None -> self.db.insert(Prog {
                    name : child.name,
                    weight : child.weight,
                    parent : Some(new_prog),
                    children : vec![],
            }
        }
    }
}

fn parse_prog(line : &str) -> ProgInfo {
    lazy_static! {
        static ref RE_PROG_INFO : regex::Regex = Regex::new(r"^(\w+) \((\d+)\)").expect("failed to compile regex");
        static ref RE_PROG_CHILDREN : regex::Regex = Regex::new(r",? (\w+)").expect("failed to compile regex");
    }

    let prog_info_captures = RE_PROG_INFO.captures_iter(line).nth(0).unwrap();

    let children_captures_iter = RE_PROG_CHILDREN.captures_iter(line);

    ProgInfo {
        name : prog_info_captures.get(1).unwrap().as_str(),
        weight : prog_info_captures[2].parse::<u32>().unwrap(),
        child_names : children_captures_iter.map(|caps| {
            caps.get(1).unwrap().as_str()
        }).collect(),
    }
}

fn solve_a(input : &str) -> u32 {
    0
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
    fn parse_no_children() {
        let input = "pbga (66)";
        assert_eq!(parse_prog(&input), ProgInfo {
            name: "pbga",
            weight: 66,
            child_names: vec![],
        });
    }

    #[test]
    fn parse_with_children() {
        let input = "fwft (72) -> ktlj, cntj, xhth";
        assert_eq!(parse_prog(&input), ProgInfo {
            name: "fwft",
            weight: 72,
            child_names: vec!["ktlj", "cntj", "xhth"],
        });
    }

    #[test]
    fn a_1() {
        let input = "blah";
        assert_eq!(solve_a(&input), 0);
    }

    #[test]
    fn b_1() {
        let input = "blah";
        assert_eq!(solve_b(&input), 0);
    }
}
