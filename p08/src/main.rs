#![feature(nll)]

use std::collections::HashMap;
use std::fmt;

#[macro_use] extern crate lazy_static;
extern crate regex;
use regex::Regex;

extern crate aoclib;
use aoclib::*;

enum ModifyOperation {
    Inc,
    Dec,
}

impl ModifyOperation {
    fn from(s : &str) -> ModifyOperation {
        match s {
            "inc" => ModifyOperation::Inc,
            "dec" => ModifyOperation::Dec,
            _ => panic!("unknown modify operation {}", s)
        }
    }
}

impl<'t> fmt::Display for ModifyOperation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(
            match self {
                &ModifyOperation::Inc => "inc",
                &ModifyOperation::Dec => "dec",
            })
    }
}

enum ConditionOperation {
    LessThan,
    GreaterThan,
    Equal,
    LessThanEqual,
    GreaterThanEqual,
    NotEqual,
}

impl ConditionOperation {
    fn from(s : &str) -> ConditionOperation {
        match s {
            "<" => ConditionOperation::LessThan,
            ">" => ConditionOperation::GreaterThan,
            "==" => ConditionOperation::Equal,
            "<=" => ConditionOperation::LessThanEqual,
            ">=" => ConditionOperation::GreaterThanEqual,
            "!=" => ConditionOperation::NotEqual,
            _ => panic!("unknown condition operation {}", s)
        }
    }

    fn apply<T>(&self, left : T, right : T) -> bool
        where T : Eq + Ord
    {
        match self {
            &ConditionOperation::LessThan => left < right,
            &ConditionOperation::GreaterThan => left > right,
            &ConditionOperation::Equal => left == right,
            &ConditionOperation::LessThanEqual => left <= right,
            &ConditionOperation::GreaterThanEqual => left >= right,
            &ConditionOperation::NotEqual => left != right,
        }
    }
}

impl<'t> fmt::Display for ConditionOperation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(
            match self {
                &ConditionOperation::LessThan => "<",
                &ConditionOperation::GreaterThan => ">",
                &ConditionOperation::Equal => "==",
                &ConditionOperation::LessThanEqual => "<=",
                &ConditionOperation::GreaterThanEqual => ">=",
                &ConditionOperation::NotEqual => "!=",
            })
    }
}

struct Instruction<'t> {
    modified_register : &'t str,
    modify_op : ModifyOperation,
    modify_by : i32,
    condition_register : &'t str,
    condition_op : ConditionOperation,
    condition_val : i32,
}

impl<'t> Instruction<'t> {
    fn from(line : &'t str) -> Instruction<'t> {
        lazy_static! {
            static ref RE_INSTRUCTION : regex::Regex = Regex::new(r"^(\w+) (inc|dec) (-?\d+) if (\w+) ([<>=!]=?) (-?\d+)$").expect("failed to compile regex");
        }

        let captures = RE_INSTRUCTION.captures_iter(line).nth(0).unwrap();
        Instruction {
            modified_register : captures.get(1).unwrap().as_str(),
            modify_op : ModifyOperation::from(captures.get(2).unwrap().as_str()),
            modify_by : captures[3].parse::<i32>().unwrap(),
            condition_register : captures.get(4).unwrap().as_str(),
            condition_op : ConditionOperation::from(captures.get(5).unwrap().as_str()),
            condition_val : captures[6].parse::<i32>().unwrap(),
        }
    }
}
impl<'t> fmt::Display for Instruction<'t> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} {} if {} {} {}", self.modified_register, self.modify_op, self.modify_by, self.condition_register, self.condition_op, self.condition_val)
    }
}

struct Program<'t> {
    instructions : Vec<Instruction<'t>>,
}

impl<'t> Program<'t> {
    fn from(input : &'t str) -> Program<'t> {
        Program {
            instructions : input.lines().map(|line| {
                Instruction::from(line)
            }).collect(),
        }
    }

    fn run<'a>(&'a self) -> ProgramExecution<'a, 't> {
        ProgramExecution {
            instructions : self.instructions.iter(),
            registers : HashMap::new(),
        }
    }
}

impl<'t> fmt::Display for Program<'t> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (i, instruction) in self.instructions.iter().enumerate() {
            if i != 0 {
                let _ = writeln!(f, "");
            }

            let _ = instruction.fmt(f);
        }

        write!(f, "")
    }
}

struct ProgramExecution<'a, 't: 'a> {
    instructions : std::slice::Iter<'a, Instruction<'t>>,
    registers : HashMap<&'t str, i32>,
}

impl<'a, 't> ProgramExecution<'a, 't> {
    fn max_value(&self) -> i32 {
        self.registers.values().map(|v| *v).max().unwrap_or(i32::min_value())
    }

    fn inc(&mut self, reg : &'t str, by : i32) {
        self.registers.insert(reg, self.registers.get(reg).unwrap_or(&0) + by);
    }
}

impl<'a, 't> Iterator for ProgramExecution<'a, 't> {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        self.instructions.next().map(|instruction| {
            let condition_reg_value = self.registers.get(instruction.condition_register).unwrap_or(&0);
            if instruction.condition_op.apply(*condition_reg_value, instruction.condition_val) {
                let by = match instruction.modify_op {
                    ModifyOperation::Inc => instruction.modify_by,
                    ModifyOperation::Dec => -instruction.modify_by,
                };

                self.inc(instruction.modified_register, by);
            }

            self.max_value()
        })
    }
}

fn solve_a(input : &str) -> i32 {
    let prog = Program::from(input);
    eprintln!("{}", prog);
    prog.run().last().unwrap_or(i32::min_value())
}

fn solve_b(input : &str) -> i32 {
    let prog = Program::from(input);
    eprintln!("{}", prog);
    prog.run().max().unwrap_or(i32::min_value())
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
        let input =
r"b inc 5 if a > 1
a inc 1 if b < 5
c dec -10 if a >= 1
c inc -20 if c == 10";
        assert_eq!(solve_a(&input), 1);
    }

    #[test]
    fn a_le() {
        let input =
r"b inc 1 if b <= 1
b inc 1 if b <= 1
b inc 1 if b <= 1";
        assert_eq!(solve_a(&input), 2);
    }

    #[test]
    fn a_ne() {
        let input =
r"b inc 1 if b != 1
b inc 5 if b != 1
b inc 6 if b != 0";
        assert_eq!(solve_a(&input), 7);
    }

    #[test]
    fn b_1() {
        let input =
r"b inc 5 if a > 1
a inc 1 if b < 5
c dec -10 if a >= 1
c inc -20 if c == 10";
        assert_eq!(solve_b(&input), 10);
    }
}
