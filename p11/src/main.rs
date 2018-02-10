#![feature(nll)]
#![feature(dyn_trait)]

extern crate aoclib;
use aoclib::*;

struct HexMover<'t> {
    pos : (i32, i32),
    steps : Vec<&'t str>,
    index : usize,
}

impl<'t> HexMover<'t> {
    fn new(steps : &'t str) -> HexMover<'t> {
        let steps = steps.split(',').collect();

        HexMover {
            pos : (0, 0),
            steps : steps,
            index : 0,
        }
    }
}

impl<'t> Iterator for HexMover<'t> {
    type Item = (i32, i32);

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.steps.len() {
            self.pos = match self.steps[self.index] {
                "n" => (self.pos.0, self.pos.1 + 2),
                "s" => (self.pos.0, self.pos.1 - 2),
                "ne" => (self.pos.0 + 2, self.pos.1 + 1),
                "se" => (self.pos.0 + 2, self.pos.1 - 1),
                "nw" => (self.pos.0 - 2, self.pos.1 + 1),
                "sw" => (self.pos.0 - 2, self.pos.1 - 1),
                _ => panic!("invalid step {}", self.steps[self.index]),
            };

            self.index += 1;

            Some(self.pos)
        } else {
            None
        }
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

    fn test_coords(input : &str, final_pos : (i32, i32)) {
        let mover = HexMover::new(input);
        assert_eq!(mover.last().unwrap(), final_pos);
    }

    #[test]
    fn a_1() {
        test_coords("ne,sw", (0, 0));
    }

    #[test]
    fn b_1() {
        let input = "blah";
        assert_eq!(solve_b(&input), 0);
    }
}
