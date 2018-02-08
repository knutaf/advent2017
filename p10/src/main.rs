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
        while self.pos >= self.ring.len() {
            self.pos -= self.ring.len();
        }

        self.skip_size += 1;

        self.ring[0] * self.ring[1]
    }

    fn reduce(&self, block_size : usize) -> Vec<u32> {
        let mut result : Vec<u32> = vec![];

        let _ = self.ring.iter().enumerate().fold(&mut result, |sofar, (i, num)| {
            if (i % block_size) == 0 {
                sofar.push(*num);
            } else {
                let last = *sofar.last().unwrap();
                *sofar.last_mut().unwrap() = last ^ num;
            }

            sofar
        }).last();

        result
    }
}

fn solve_a(input : &str, ring_size : u32) -> u32 {
    let mut ring = Ring::new(ring_size);

    input.split(',').map(|num_str| {
        ring.advance(num_str.parse::<u8>().unwrap_or(0) as usize)
    }).last().unwrap() as u32
}

fn solve_b(input : &str) -> String {
    const SUFFIX_LENGTHS : [u8 ; 5] = [17u8, 31u8, 73u8, 47u8, 23u8];
    const NUM_ROUNDS : u32 = 64;
    const BLOCK_SIZE : u32 = 16;

    let mut ring = Ring::new(256);

    for _ in 0 .. NUM_ROUNDS {
        for b in input.bytes() {
            let _ = ring.advance(b as usize);
        }

        for b in SUFFIX_LENGTHS.iter() {
            let _ = ring.advance(*b as usize);
        }
    }

    String::from("")
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
    fn reduce_default_2() {
        let ring = Ring::new(10);
        assert_eq!(ring.reduce(2), vec![1, 1, 1, 1, 1]);
    }

    #[test]
    fn reduce_default_3() {
        let ring = Ring::new(9);
        assert_eq!(ring.reduce(3), vec![3, 2, 9]);
    }

    #[test]
    fn reduce_size_1() {
        let ring = Ring::new(5);
        assert_eq!(ring.reduce(1), vec![0, 1, 2, 3, 4]);
    }

    #[test]
    fn b_1() {
        let input = "blah";
        assert_eq!(solve_b(&input), "");
    }
}
