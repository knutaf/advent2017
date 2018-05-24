#![feature(nll)]

use std::fmt;
use std::collections::HashMap;

#[macro_use] extern crate lazy_static;
extern crate regex;
use regex::Regex;

extern crate aoclib;
use aoclib::*;

#[derive(PartialEq)]
enum Direction {
    Left,
    Right,
}

struct Action {
    condition_val : bool,
    write_val : bool,
    dir : Direction,
    next_state : char,
}

struct State {
    actions : [Action ; 2],
}

struct Machine {
    initial_state : char,
    checksum_after : usize,
    states : HashMap<char, State>,
}

struct Execution<'m> {
    machine : &'m Machine,
    state : char,
    pos : usize,
    steps_taken : usize,
    num_ones : usize,
    tape : Vec<bool>,
}

impl Direction {
    fn parse(input : &str) -> Direction {
        if input == "left" {
            Direction::Left
        } else if input == "right" {
            Direction::Right
        } else {
            panic!("unknown direction {}", input);
        }
    }
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self == &Direction::Left {
            write!(f, "left")
        } else {
            write!(f, "right")
        }
    }
}

impl Action {
    fn load<'t>(lines : &mut impl Iterator<Item = &'t str>) -> Action {
        lazy_static! {
            static ref RE_CONDITION : regex::Regex = Regex::new(r"^\s*If the current value is (\d+):$").expect("failed to compile regex");
            static ref RE_WRITE : regex::Regex = Regex::new(r"^\s*- Write the value (\d+)\.$").expect("failed to compile regex");
            static ref RE_MOVE : regex::Regex = Regex::new(r"^\s*- Move one slot to the (\w+)\.$").expect("failed to compile regex");
            static ref RE_NEXT_STATE : regex::Regex = Regex::new(r"^\s*- Continue with state (\w)\.$").expect("failed to compile regex");
        }

        let mut condition_val = false;
        let mut write_val = false;
        let mut dir = Direction::Left;
        let mut next_state = 'a';

        while next_state == 'a' {
            if let Some(line) = lines.next() {
                if let Some(captures) = RE_CONDITION.captures_iter(line).next() {
                    condition_val = captures.get(1).unwrap().as_str().parse::<u8>().unwrap() != 0;
                } else if let Some(captures) = RE_WRITE.captures_iter(line).next() {
                    write_val = captures.get(1).unwrap().as_str().parse::<u8>().unwrap() != 0;
                } else if let Some(captures) = RE_MOVE.captures_iter(line).next() {
                    dir = Direction::parse(captures.get(1).unwrap().as_str());
                } else if let Some(captures) = RE_NEXT_STATE.captures_iter(line).next() {
                    next_state = captures.get(1).unwrap().as_str().chars().nth(0).unwrap();
                }
            } else {
                break;
            }
        }

        Action {
            condition_val,
            write_val,
            dir,
            next_state
        }
    }
}

impl fmt::Display for Action {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
r"  If the current value is {}:
    - Write the value {}.
    - Move one slot to the {}.
    - Continue with state {}.
",
            if self.condition_val { 1 } else { 0 },
            if self.write_val { 1 } else { 0 },
            self.dir,
            self.next_state)
    }
}

impl State {
    fn load<'t>(lines : &mut impl Iterator<Item = &'t str>) -> State {
        let action0 = Action::load(lines);
        let action1 = Action::load(lines);

        State {
            actions : [action0, action1],
        }
    }

    fn get_action_for_condition(&self, condition_val : bool) -> &Action {
        self.actions.iter().find(|action| {
            action.condition_val == condition_val
        }).unwrap()
    }
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut ret = write!(f, "");
        for action in self.actions.iter() {
            ret = write!(f, "{}", action)
        }
        ret
    }
}

impl Machine {
    fn load(input : &str) -> Machine {
        lazy_static! {
            static ref RE_BEGIN : regex::Regex = Regex::new(r"^Begin in state (\w)\.$").expect("failed to compile regex");
            static ref RE_CHECKSUM : regex::Regex = Regex::new(r"^Perform a diagnostic checksum after (\d+) steps\.$").expect("failed to compile regex");
            static ref RE_START_STATE : regex::Regex = Regex::new(r"^In state (\w):$").expect("failed to compile regex");
        }

        let mut initial_state = 'a';
        let mut checksum_after = 0;
        let mut states : HashMap<char, State> = HashMap::new();

        let mut lines = input.lines();
        while let Some(line) = lines.next() {
            if let Some(captures) = RE_BEGIN.captures_iter(line).next() {
                initial_state = captures.get(1).unwrap().as_str().chars().nth(0).unwrap();
            } else if let Some(captures) = RE_CHECKSUM.captures_iter(line).next() {
                checksum_after = captures.get(1).unwrap().as_str().parse::<usize>().unwrap();
            } else if let Some(captures) = RE_START_STATE.captures_iter(line).next() {
                let state_name = captures.get(1).unwrap().as_str().chars().nth(0).unwrap();
                states.insert(state_name, State::load(&mut lines));
            }
        }

        Machine {
            initial_state,
            checksum_after,
            states,
        }
    }
}

impl fmt::Display for Machine {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut ret;
        ret = write!(f,
r"Begin in state {}.
Perform a diagnostic checksum after {} steps.

",
                  self.initial_state, self.checksum_after);

        for (name, state) in self.states.iter() {
            ret = write!(f, "In state {}:\n{}\n", name, state);
        }
        ret
    }
}

impl<'m> Execution<'m> {
    fn new(machine : &'m Machine) -> Execution<'m> {
        Execution {
            machine,
            state : machine.initial_state,
            pos : machine.checksum_after - 1,
            steps_taken : 0,
            num_ones : 0,
            tape : vec![false ; machine.checksum_after * 2],
        }
    }
}

impl<'m> Iterator for Execution<'m> {
    type Item = ();

    fn next(&mut self) -> Option<Self::Item> {
        if self.steps_taken < self.machine.checksum_after {
            self.steps_taken += 1;

            let state = self.machine.states.get(&self.state).unwrap();
            let cursor_val = self.tape.get_mut(self.pos).unwrap();

            let action = state.get_action_for_condition(*cursor_val);

            if *cursor_val != action.write_val {
                if action.write_val == true {
                    self.num_ones += 1;
                } else {
                    self.num_ones -= 1;
                }
            }

            *cursor_val = action.write_val;

            if action.dir == Direction::Left {
                self.pos -= 1;
            } else {
                self.pos += 1;
            }

            self.state = action.next_state;

            Some(())
        } else {
            None
        }
    }
}

fn solve_a(input : &str) -> usize {
    let machine = Machine::load(input);
    eprintln!("machine:\n{}", machine);
    let mut execution = Execution::new(&machine);
    aoclib::consume_iterator(&mut execution);
    execution.num_ones
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
r"Begin in state A.
Perform a diagnostic checksum after 6 steps.

In state A:
  If the current value is 0:
    - Write the value 1.
    - Move one slot to the right.
    - Continue with state B.
  If the current value is 1:
    - Write the value 0.
    - Move one slot to the left.
    - Continue with state B.

In state B:
  If the current value is 0:
    - Write the value 1.
    - Move one slot to the left.
    - Continue with state A.
  If the current value is 1:
    - Write the value 1.
    - Move one slot to the right.
    - Continue with state A.";

        assert_eq!(solve_a(&input), 3);
    }

    #[test]
    fn b_1() {
        let input = "blah";
        assert_eq!(solve_b(&input), 0);
    }
}
