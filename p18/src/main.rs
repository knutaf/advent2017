#![feature(nll)]

use std::fmt;

#[macro_use] extern crate lazy_static;
extern crate regex;
use regex::Regex;

extern crate aoclib;
use aoclib::*;

enum RegisterOrValue {
    Reg(char),
    Val(i32),
}

enum Instruction {
    Snd(RegisterOrValue),
    Set(char, RegisterOrValue),
    Add(char, RegisterOrValue),
    Mul(char, RegisterOrValue),
    Mod(char, RegisterOrValue),
    Rcv(RegisterOrValue),
    Jgz(char, RegisterOrValue),
}

struct Program {
    instructions : Vec<Instruction>,
}

impl RegisterOrValue {
    fn from(input : &str) -> RegisterOrValue {
        lazy_static! {
            static ref RE_REGISTER : regex::Regex = Regex::new(r"^([a-zA-Z])$").expect("failed to compile regex");
            static ref RE_VALUE : regex::Regex = Regex::new(r"^(-?\d+)$").expect("failed to compile regex");
        }

        if let Some(captures) = RE_REGISTER.captures_iter(input).next() {
            RegisterOrValue::Reg(captures.get(1).unwrap().as_str().chars().nth(0).unwrap())
        } else if let Some(captures) = RE_VALUE.captures_iter(input).next() {
            RegisterOrValue::Val(captures.get(1).unwrap().as_str().parse::<i32>().unwrap())
        } else {
            panic!("invalid register or value {}", input);
        }
    }
}

impl fmt::Display for RegisterOrValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &RegisterOrValue::Reg(a) => write!(f, "{}", a),
            &RegisterOrValue::Val(a) => write!(f, "{}", a),
        }
    }
}

impl Instruction {
    fn from(input : &str) -> Instruction {
        lazy_static! {
            static ref RE_SND : regex::Regex = Regex::new(r"^snd (.*)$").expect("failed to compile regex");
            static ref RE_SET : regex::Regex = Regex::new(r"^set ([a-zA-Z]) (.*)$").expect("failed to compile regex");
            static ref RE_ADD : regex::Regex = Regex::new(r"^add ([a-zA-Z]) (.*)$").expect("failed to compile regex");
            static ref RE_MUL : regex::Regex = Regex::new(r"^mul ([a-zA-Z]) (.*)$").expect("failed to compile regex");
            static ref RE_MOD : regex::Regex = Regex::new(r"^mod ([a-zA-Z]) (.*)$").expect("failed to compile regex");
            static ref RE_RCV : regex::Regex = Regex::new(r"^rcv (.*)$").expect("failed to compile regex");
            static ref RE_JGZ : regex::Regex = Regex::new(r"^jgz ([a-zA-Z]) (.*)$").expect("failed to compile regex");
        }

        if let Some(captures) = RE_SND.captures_iter(input).next() {
            Instruction::Snd(RegisterOrValue::from(captures.get(1).unwrap().as_str()))
        } else if let Some(captures) = RE_SET.captures_iter(input).next() {
            Instruction::Set(captures.get(1).unwrap().as_str().chars().nth(0).unwrap(), RegisterOrValue::from(captures.get(2).unwrap().as_str()))
        } else if let Some(captures) = RE_ADD.captures_iter(input).next() {
            Instruction::Add(captures.get(1).unwrap().as_str().chars().nth(0).unwrap(), RegisterOrValue::from(captures.get(2).unwrap().as_str()))
        } else if let Some(captures) = RE_MUL.captures_iter(input).next() {
            Instruction::Mul(captures.get(1).unwrap().as_str().chars().nth(0).unwrap(), RegisterOrValue::from(captures.get(2).unwrap().as_str()))
        } else if let Some(captures) = RE_MOD.captures_iter(input).next() {
            Instruction::Mod(captures.get(1).unwrap().as_str().chars().nth(0).unwrap(), RegisterOrValue::from(captures.get(2).unwrap().as_str()))
        } else if let Some(captures) = RE_JGZ.captures_iter(input).next() {
            Instruction::Jgz(captures.get(1).unwrap().as_str().chars().nth(0).unwrap(), RegisterOrValue::from(captures.get(2).unwrap().as_str()))
        } else if let Some(captures) = RE_RCV.captures_iter(input).next() {
            Instruction::Rcv(RegisterOrValue::from(captures.get(1).unwrap().as_str()))
        } else {
            panic!("invalid move {}", input);
        }
    }
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Instruction::Snd(ref a) => write!(f, "snd {}", a),
            &Instruction::Set(ref a, ref b) => write!(f, "set {} {}", a, b),
            &Instruction::Add(ref a, ref b) => write!(f, "add {} {}", a, b),
            &Instruction::Mul(ref a, ref b) => write!(f, "mul {} {}", a, b),
            &Instruction::Mod(ref a, ref b) => write!(f, "mod {} {}", a, b),
            &Instruction::Rcv(ref a) => write!(f, "rcv {}", a),
            &Instruction::Jgz(ref a, ref b) => write!(f, "jgz {} {}", a, b),
        }
    }
}

impl Program {
    fn load(input : &str) -> Program {
        Program {
            instructions : input.lines().map(Instruction::from).collect(),
        }
    }
}

impl fmt::Display for Program {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut ret = write!(f, "");
        for inst in self.instructions.iter() {
            ret = write!(f, "{}\n", inst);
        }
        ret
    }
}

fn solve_a(input : &str) -> u32 {
    let prog = Program::load(input);
    eprintln!("prog: {}", prog);
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
    fn a_given() {
        let input =
r"set a 1
add a 2
mul a a
mod a 5
snd a
set a 0
rcv a
jgz a -1
set a 1
jgz a -2";
        assert_eq!(solve_a(&input), 4);
    }

    #[test]
    fn b_1() {
        let input = "blah";
        assert_eq!(solve_b(&input), 0);
    }
}
