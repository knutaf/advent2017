#![feature(nll)]

extern crate aoclib;
use aoclib::*;

struct CircularBuffer {
    state : Vec<u32>,
    step_size : u32,
    position : usize,
    insertion : u32,
    max_insertions : u32,
}

struct ZeroTrackingCircularBuffer {
    length : usize,
    step_size : u32,
    position : usize,
    insertion : u32,
    max_insertions : u32,
    after_zero : u32,
}

impl CircularBuffer {
    fn new(step_size : u32, max_insertions : u32) -> CircularBuffer {
        let mut ret = CircularBuffer {
            state : Vec::with_capacity((max_insertions as usize) + 1),
            step_size : step_size,
            position : 0,
            insertion : 1,
            max_insertions : max_insertions,
        };

        ret.state.push(0);

        ret
    }
}

impl ZeroTrackingCircularBuffer {
    fn new(step_size : u32, max_insertions : u32) -> ZeroTrackingCircularBuffer {
        ZeroTrackingCircularBuffer {
            length : 1,
            step_size : step_size,
            position : 0,
            insertion : 1,
            max_insertions : max_insertions,
            after_zero : 0,
        }
    }
}

impl Iterator for CircularBuffer {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        let ret;
        if self.insertion <= self.max_insertions {
            self.position = ((self.position + (self.step_size as usize)) % self.state.len()) + 1;

            self.state.insert(self.position, self.insertion);
            self.insertion += 1;

            ret = Some(self.state[(self.position + 1) % self.state.len()]);
        } else {
            ret = None;
        }

        eprintln!("buf: {:?}", self.state);

        ret
    }
}

impl Iterator for ZeroTrackingCircularBuffer {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        let ret;
        if self.insertion <= self.max_insertions {
            self.position = ((self.position + (self.step_size as usize)) % self.length) + 1;

            if self.position == 1 {
                self.after_zero = self.insertion;
            }

            self.length += 1;
            self.insertion += 1;

            ret = Some(self.after_zero);
        } else {
            ret = None;
        }

        ret
    }
}

fn get_num_after(step_size : u32, max_insertions : u32) -> u32 {
    let buf = CircularBuffer::new(step_size, max_insertions);
    buf.last().unwrap()
}

fn get_num_after_zero(step_size : u32, max_insertions : u32) -> u32 {
    let buf = ZeroTrackingCircularBuffer::new(step_size, max_insertions);
    buf.last().unwrap()
}

fn solve_a(input : &str) -> u32 {
    const MAX_INSERTIONS_A : u32 = 2017;
    let step_size = input.parse::<u32>().unwrap();
    get_num_after(step_size, MAX_INSERTIONS_A)
}

fn solve_b(input : &str) -> u32 {
    const MAX_INSERTIONS_B : u32 = 50000000;
    let step_size = input.parse::<u32>().unwrap();
    get_num_after_zero(step_size, MAX_INSERTIONS_B)
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
        assert_eq!(get_num_after(3, 9), 5);
        assert_eq!(get_num_after(3, 2017), 638);
    }

    #[test]
    fn b_given() {
        assert_eq!(get_num_after_zero(3, 4), 2);
        assert_eq!(get_num_after_zero(3, 5), 5);
        assert_eq!(get_num_after_zero(3, 9), 9);
    }
}
