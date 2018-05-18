#![feature(nll)]

use std::fmt;

#[macro_use] extern crate lazy_static;
extern crate regex;
use regex::Regex;

extern crate aoclib;
use aoclib::*;

const NUM_EXECUTIONS_B : i64 = 2;

enum RegisterOrValue {
    Reg(char),
    Val(i64),
}

enum Instruction {
    Snd(RegisterOrValue),
    Set(char, RegisterOrValue),
    Add(char, RegisterOrValue),
    Mul(char, RegisterOrValue),
    Mod(char, RegisterOrValue),
    Rcv(char),
    Jgz(RegisterOrValue, RegisterOrValue),
}

struct Program {
    instructions : Vec<Instruction>,
}

struct RegisterHolder {
    registers : [i64 ; ((('z' as u8) - ('a' as u8)) + 1) as usize],
}

struct Execution<'t> {
    instructions : &'t Vec<Instruction>,
    position : usize,
    registers : RegisterHolder,
    last_freq : i64,
    last_recovery : Option<i64>,
}

struct ExecutionStep {
    is_blocked : bool,
    new_snd_opt : Option<i64>,
}

struct ExecutionB<'t> {
    instructions : &'t Vec<Instruction>,
    position : usize,
    registers : RegisterHolder,
    rcv_queue : Vec<i64>,
    snd_count : u32,
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
            RegisterOrValue::Val(captures.get(1).unwrap().as_str().parse::<i64>().unwrap())
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
            static ref RE_RCV : regex::Regex = Regex::new(r"^rcv ([a-zA-Z])$").expect("failed to compile regex");
            static ref RE_JGZ : regex::Regex = Regex::new(r"^jgz (.*) (.*)$").expect("failed to compile regex");
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
            Instruction::Jgz(RegisterOrValue::from(captures.get(1).unwrap().as_str()), RegisterOrValue::from(captures.get(2).unwrap().as_str()))
        } else if let Some(captures) = RE_RCV.captures_iter(input).next() {
            Instruction::Rcv(captures.get(1).unwrap().as_str().chars().nth(0).unwrap())
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

    fn execute<'t>(&'t self) -> Execution<'t> {
        Execution::new(&self.instructions)
    }

    fn execute_b<'t>(&'t self, program_id : i64) -> ExecutionB<'t> {
        ExecutionB::new(&self.instructions, program_id)
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

impl RegisterHolder {
    fn new() -> RegisterHolder {
        RegisterHolder {
            registers : [0 ; ((('z' as u8) - ('a' as u8)) + 1) as usize],
        }
    }

    fn get_reg_mut(&mut self, reg : char) -> &mut i64 {
        &mut self.registers[reg as usize - 'a' as usize]
    }

    fn get_reg(&self, reg : char) -> &i64 {
        &self.registers[reg as usize - 'a' as usize]
    }

    fn evaluate(&self, rv : &RegisterOrValue) -> i64 {
        match rv {
            &RegisterOrValue::Reg(r) => {
                *self.get_reg(r)
            },
            &RegisterOrValue::Val(v) => {
                v
            }
        }
    }
}

impl<'t> Execution<'t> {
    fn new(instructions : &'t Vec<Instruction>) -> Execution<'t> {
        Execution {
            instructions : instructions,
            position : 0,
            registers : RegisterHolder::new(),
            last_freq : 0,
            last_recovery : None,
        }
    }
}

impl<'t> Iterator for Execution<'t> {
    type Item = Option<i64>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.position < self.instructions.len() {
            let inst = &self.instructions[self.position];
            eprintln!("{}: {}", self.position, inst);

            match inst {
                &Instruction::Snd(ref rv) => {
                    self.last_freq = self.registers.evaluate(&rv);
                    eprintln!("  last_freq = {}", self.last_freq);
                }
                &Instruction::Set(ref reg, ref rv) => {
                    eprintln!("  {} <= {}", reg, self.registers.evaluate(&rv));
                    *self.registers.get_reg_mut(*reg) = self.registers.evaluate(&rv);
                }
                &Instruction::Add(ref reg, ref rv) => {
                    eprintln!("  add {} {} ({})", *self.registers.get_reg(*reg), self.registers.evaluate(&rv), *self.registers.get_reg(*reg) + self.registers.evaluate(&rv));
                    *self.registers.get_reg_mut(*reg) = *self.registers.get_reg(*reg) + self.registers.evaluate(&rv);
                }
                &Instruction::Mul(ref reg, ref rv) => {
                    eprintln!("  mul {} {} ({})", *self.registers.get_reg(*reg), self.registers.evaluate(&rv), *self.registers.get_reg(*reg) * self.registers.evaluate(&rv));
                    *self.registers.get_reg_mut(*reg) = *self.registers.get_reg(*reg) * self.registers.evaluate(&rv);
                }
                &Instruction::Mod(ref reg, ref rv) => {
                    eprintln!("  mod {} {} ({})", *self.registers.get_reg(*reg), self.registers.evaluate(&rv), *self.registers.get_reg(*reg) % self.registers.evaluate(&rv));
                    *self.registers.get_reg_mut(*reg) = *self.registers.get_reg(*reg) % self.registers.evaluate(&rv);
                }
                &Instruction::Rcv(ref reg) => {
                    if self.registers.evaluate(&RegisterOrValue::Reg(*reg)) != 0 {
                        eprintln!("  set to {}", self.last_freq);
                        self.last_recovery = Some(self.last_freq);
                    } else {
                        eprintln!("  skip");
                    }
                }
                &Instruction::Jgz(..) => {},
            }

            self.position = match inst {
                &Instruction::Jgz(ref cond, ref jump_offset) => {
                    if self.registers.evaluate(&cond) > 0 {
                        let offset = self.registers.evaluate(&jump_offset);
                        if offset >= 0 {
                            self.position + (offset as usize)
                        } else {
                            if ((offset * -1) as usize) <= self.position {
                                ((self.position as i64) + offset) as usize
                            } else {
                                self.instructions.len()
                            }
                        }
                    } else {
                        self.position + 1
                    }
                },
                &Instruction::Snd(..) |
                &Instruction::Set(..) |
                &Instruction::Add(..) |
                &Instruction::Mul(..) |
                &Instruction::Mod(..) |
                &Instruction::Rcv(..) => {
                    self.position + 1
                },
            };

            Some(self.last_recovery)
        } else {
            None
        }
    }
}

impl<'t> ExecutionB<'t> {
    fn new(instructions : &'t Vec<Instruction>, program_id : i64) -> ExecutionB<'t> {
        let mut exec = ExecutionB {
            instructions : instructions,
            position : 0,
            registers : RegisterHolder::new(),
            rcv_queue : Vec::new(),
            snd_count : 0,
        };

        *exec.registers.get_reg_mut('p') = program_id;

        exec
    }

    fn rcv(&mut self, value : i64) {
        self.rcv_queue.insert(0, value);
    }
}

impl<'t> Iterator for ExecutionB<'t> {
    type Item = ExecutionStep;

    fn next(&mut self) -> Option<Self::Item> {
        if self.position < self.instructions.len() {
            let inst = &self.instructions[self.position];
            eprintln!("{}: {}", self.position, inst);

            let mut is_blocked = false;
            let mut new_snd_opt = None;

            match inst {
                &Instruction::Snd(ref rv) => {
                    self.snd_count += 1;
                    new_snd_opt = Some(self.registers.evaluate(&rv));
                    eprintln!("  sending {:?}", new_snd_opt.as_ref());
                }
                &Instruction::Set(ref reg, ref rv) => {
                    eprintln!("  {} <= {}", reg, self.registers.evaluate(&rv));
                    *self.registers.get_reg_mut(*reg) = self.registers.evaluate(&rv);
                }
                &Instruction::Add(ref reg, ref rv) => {
                    eprintln!("  add {} {} ({})", *self.registers.get_reg(*reg), self.registers.evaluate(&rv), *self.registers.get_reg(*reg) + self.registers.evaluate(&rv));
                    *self.registers.get_reg_mut(*reg) = *self.registers.get_reg(*reg) + self.registers.evaluate(&rv);
                }
                &Instruction::Mul(ref reg, ref rv) => {
                    eprintln!("  mul {} {} ({})", *self.registers.get_reg(*reg), self.registers.evaluate(&rv), *self.registers.get_reg(*reg) * self.registers.evaluate(&rv));
                    *self.registers.get_reg_mut(*reg) = *self.registers.get_reg(*reg) * self.registers.evaluate(&rv);
                }
                &Instruction::Mod(ref reg, ref rv) => {
                    eprintln!("  mod {} {} ({})", *self.registers.get_reg(*reg), self.registers.evaluate(&rv), *self.registers.get_reg(*reg) % self.registers.evaluate(&rv));
                    *self.registers.get_reg_mut(*reg) = *self.registers.get_reg(*reg) % self.registers.evaluate(&rv);
                }
                &Instruction::Rcv(ref reg) => {
                    if let Some(val) = self.rcv_queue.pop() {
                        *self.registers.get_reg_mut(*reg) = val;
                        eprintln!("  rcv {} into {}", val, reg);
                    } else {
                        is_blocked = true;
                        eprintln!("  blocked on rcv into {}", reg);
                    }
                }
                &Instruction::Jgz(..) => {},
            }

            self.position =
                if is_blocked {
                    self.position
                } else {
                    match inst {
                        &Instruction::Jgz(ref cond, ref jump_offset) => {
                            if self.registers.evaluate(&cond) > 0 {
                                let offset = self.registers.evaluate(&jump_offset);
                                if offset >= 0 {
                                    self.position + (offset as usize)
                                } else {
                                    if ((offset * -1) as usize) <= self.position {
                                        ((self.position as i64) + offset) as usize
                                    } else {
                                        self.instructions.len()
                                    }
                                }
                            } else {
                                self.position + 1
                            }
                        },
                        &Instruction::Snd(..) |
                        &Instruction::Set(..) |
                        &Instruction::Add(..) |
                        &Instruction::Mul(..) |
                        &Instruction::Mod(..) |
                        &Instruction::Rcv(..) => {
                            self.position + 1
                        },
                    }
                };

            Some(ExecutionStep {
                is_blocked,
                new_snd_opt,
            })
        } else {
            None
        }
    }
}

fn run_duet(prog : &Program) -> Vec<ExecutionB> {
    let mut execs = (0 .. NUM_EXECUTIONS_B).map(|i| {
        prog.execute_b(i)
    }).collect::<Vec<ExecutionB>>();

    let mut made_progress = true;
    while made_progress {
        made_progress = false;

        for i in 0 .. execs.len() {
            if let Some(step) = execs[i].next() {
                made_progress = made_progress || !step.is_blocked;
                if let Some(new_snd) = step.new_snd_opt {
                    made_progress = true;
                    let execs_len = execs.len();
                    execs[(i + 1) % execs_len].rcv(new_snd);
                }
            }
        }
    }

    execs
}

fn solve_a(input : &str) -> i64 {
    let prog = Program::load(input);
    eprintln!("prog: {}", prog);

    prog.execute().find(|last_rcv_opt| {
        last_rcv_opt.is_some()
    }).unwrap().unwrap()
}

fn solve_b(input : &str) -> u32 {
    let prog = Program::load(input);
    eprintln!("prog: {}", prog);

    run_duet(&prog)[1].snd_count
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
    fn b_given() {
        let input =
r"snd 1
snd 2
snd p
rcv a
rcv b
rcv c
rcv d";
        assert_eq!(solve_b(&input), 3);
    }
}
