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

    fn take_jump_a(&mut self) -> i32 {
        let ret = self.offset + self.modifier;
        self.modifier = self.modifier + 1;
        ret
    }

    fn take_jump_b(&mut self) -> i32 {
        let ret = self.offset + self.modifier;

        if ret >= 3 {
            self.modifier = self.modifier - 1;
        } else {
            self.modifier = self.modifier + 1;
        }

        ret
    }
}

struct Program {
    instructions : Vec<Instruction>,
    ip : i32,
    mode_a : bool,
}

impl Program {
    fn new(instructions : Vec<Instruction>, mode_a : bool) -> Program {
        Program {
            instructions : instructions,
            ip : 0,
            mode_a: mode_a,
        }
    }

    fn from_input(input : &str, mode_a : bool) -> Program {
        Program::new(input.lines().map(|line| {
            Instruction::new(line.parse::<i32>().unwrap())
        }).collect(), mode_a)
    }
}

impl Iterator for Program {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.ip >= 0 && self.ip < (self.instructions.len() as i32) {
            {
                let instruction = self.instructions.get_mut(self.ip as usize).unwrap();
                self.ip = self.ip +
                    if self.mode_a {
                        instruction.take_jump_a()
                    } else {
                        instruction.take_jump_b()
                    };
            }

            if self.ip >= 0 && self.ip < (self.instructions.len() as i32) {
                Some(self.ip)
            } else {
                None
            }
        } else {
            None
        }
    }
}

fn solve_a(input : &str) -> u32 {
    let program = Program::from_input(input, true);
    (program.count() + 1) as u32
}

fn solve_b(input : &str) -> u32 {
    let program = Program::from_input(input, false);
    (program.count() + 1) as u32
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
        let input =
r"0
3
0
1
-3";

        assert_eq!(solve_b(&input), 10);
    }

    #[test]
    fn b_2() {
        let input = r"0";
        assert_eq!(solve_b(&input), 2);
    }

    #[test]
    fn b_3() {
        let input = r"1";
        assert_eq!(solve_b(&input), 1);
    }

    #[test]
    fn b_4() {
        let input =
r"0
0";
        assert_eq!(solve_b(&input), 4);
    }

    #[test]
    fn b_5() {
        let input =
r"1
-1";
        assert_eq!(solve_b(&input), 3);
    }

    #[test]
    fn b_6() {
        let input =
r"3
0
2
-3";
        assert_eq!(solve_b(&input), 4);
    }
}
