extern crate aoclib;
use aoclib::*;

struct Instruction {
    offset : i32,
    modifier : i32,
}

impl Instruction {
    fn new(offset : i32) -> Instruction {
        Instruction {
            offset : offset,
            modifier : 0,
        }
    }

    fn take_jump(&mut self) -> i32 {
        self.modifier = self.modifier + 1;
        self.offset + self.modifier - 1
    }
}

struct Program {
    instructions : Vec<Instruction>,
    ip : i32,
}

impl Program {
    fn new(instructions : Vec<Instruction>) -> Program {
        Program {
            instructions : instructions,
            ip : 0,
        }
    }
}

impl Iterator for Program {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.ip >= 0 && self.ip < (self.instructions.len() as i32) {
            let next_ip = self.ip + self.instructions.get_mut(self.ip as usize).unwrap().take_jump();
            self.ip = next_ip;

            if self.ip >= 0 && self.ip < (self.instructions.len() as i32) {
                Some(next_ip)
            } else {
                None
            }
        } else {
            None
        }
    }
}

fn solve_a(input : &str) -> u32 {
    let instructions = input.lines().map(|line| {
        Instruction::new(line.parse::<i32>().unwrap())
    }).collect();

    let program = Program::new(instructions);
    (program.count() + 1) as u32
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
    fn a_1() {
        let input =
r"0
3
0
1
-3";

        assert_eq!(solve_a(&input), 5);
    }

    #[test]
    fn a_2() {
        let input = r"0";
        assert_eq!(solve_a(&input), 2);
    }

    #[test]
    fn a_3() {
        let input = r"1";
        assert_eq!(solve_a(&input), 1);
    }

    #[test]
    fn a_4() {
        let input =
r"0
0";
        assert_eq!(solve_a(&input), 4);
    }

    #[test]
    fn a_5() {
        let input =
r"1
-1";
        assert_eq!(solve_a(&input), 3);
    }

    #[test]
    fn b_1() {
        let input = "blah";
        assert_eq!(solve_b(&input), 0);
    }
}
