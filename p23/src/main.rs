#![feature(nll)]

extern crate aoclib;
use aoclib::*;
use aoclib::aocisa::*;

struct Execution<'p> {
    program : &'p Program,
    ip : usize,
    registers : RegisterHolder,
    num_muls : u32,
}

impl<'p> Execution<'p> {
    fn new(program : &'p Program) -> Execution<'p> {
        Execution {
            program,
            ip : 0,
            registers : RegisterHolder::new(),
            num_muls : 0,
        }
    }
}

impl<'p> Iterator for Execution<'p> {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.ip < self.program.instructions.len() {
            let inst = &self.program.instructions[self.ip];
            self.registers.apply_instruction(inst);

            self.ip = self.registers.get_next_ip(inst, self.ip);

            if let &Instruction::Mul(_, _) = inst {
                self.num_muls += 1;
            }

            Some(self.num_muls)
        } else {
            None
        }
    }
}

fn solve_a(input : &str) -> u32 {
    let program = Program::load(&input);
    let exec = Execution::new(&program);
    exec.last().unwrap()
}

fn solve_b(input : &str) -> i64 {
    let program = Program::load(&input);
    let mut exec = Execution::new(&program);
    *exec.registers.get_reg_mut('a') = 1;
    aoclib::consume_iterator(&mut exec);
    *exec.registers.get_reg('h')
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
r"mul a 1
mul a 1
mul a 1";
        assert_eq!(solve_a(&input), 3);
    }

    #[test]
    fn b_1() {
        let input = "blah";
        assert_eq!(solve_b(&input), 0);
    }
}
