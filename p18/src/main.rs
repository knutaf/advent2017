#![feature(nll)]

extern crate aoclib;
use aoclib::*;
use aoclib::aocisa::*;

const NUM_EXECUTIONS_B : i64 = 2;

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

trait P18 {
    fn execute<'t>(&'t self) -> Execution<'t>;
    fn execute_b<'t>(&'t self, program_id : i64) -> ExecutionB<'t>;
}

impl P18 for Program {
    fn execute<'t>(&'t self) -> Execution<'t> {
        Execution::new(&self.instructions)
    }

    fn execute_b<'t>(&'t self, program_id : i64) -> ExecutionB<'t> {
        ExecutionB::new(&self.instructions, program_id)
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
                &Instruction::Rcv(ref reg) => {
                    if self.registers.evaluate(&RegisterOrValue::Reg(*reg)) != 0 {
                        eprintln!("  set to {}", self.last_freq);
                        self.last_recovery = Some(self.last_freq);
                    } else {
                        eprintln!("  skip");
                    }
                }
                &Instruction::Jgz(..) => {},
                &Instruction::Set(..) |
                &Instruction::Add(..) |
                &Instruction::Mul(..) |
                &Instruction::Mod(..) => {
                    self.registers.apply_instruction(inst);
                },
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
                    let offset = self.registers.get_next_ip_offset(inst);

                    if offset >= 0 {
                        self.position + (offset as usize)
                    } else {
                        if ((offset * -1) as usize) <= self.position {
                            ((self.position as i64) + offset) as usize
                        } else {
                            self.instructions.len()
                        }
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
