extern crate aoclib;
use aoclib::*;

struct Ring {
    ring : Vec<u32>,
    pos : usize,
    skip_size : usize,
}

impl Ring {
    fn new(length : u32) -> Ring {
        Ring {
            ring : (0u32 .. length).collect(),
            pos : 0,
            skip_size : 0,
        }
    }

    fn advance(&mut self, length : usize) -> u32 {
        aoclib::reverse_circular_vec_segment(&mut self.ring, self.pos, length);

        self.pos += length + self.skip_size;
        if self.pos >= self.ring.len() {
            self.pos -= self.ring.len();
        }

        self.skip_size += 1;

        self.ring[0] * self.ring[1]
    }
}

fn solve_a(input : &str, ring_size : u32) -> u32 {
    let mut ring = Ring::new(ring_size);

    input.split(',').map(|num_str| {
        ring.advance(num_str.parse::<u8>().unwrap_or(0) as usize)
    }).last().unwrap() as u32
}

fn solve_b(input : &str) -> u32 {
    0
}

fn main() {
    let input = read_all_stdin();
    //eprintln!("input: {}", input);

    if aoclib::should_solve_puzzle_a() {
        println!("answer: {}", solve_a(&input, 256));
    } else {
        println!("answer: {}", solve_b(&input));
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn a_empty() {
        let input = "";
        assert_eq!(solve_a(&input, 256), 0);
    }

    #[test]
    fn a_1() {
        let input = "5";
        assert_eq!(solve_a(&input, 256), 12);
    }

    #[test]
    fn a_given() {
        let input = "3,4,1,5";
        assert_eq!(solve_a(&input, 5), 12);
    }

    #[test]
    fn b_1() {
        let input = "blah";
        assert_eq!(solve_b(&input), 0);
    }
}
