extern crate aoclib;
use aoclib::*;

fn solve_a(input : &str, ring_size : u32) -> u32 {
    let mut ring = aoclib::knot_hash::Ring::new(ring_size);

    input.split(',').map(|num_str| {
        ring.advance(num_str.parse::<u8>().unwrap_or(0) as usize)
    }).last().unwrap() as u32
}

fn solve_b(input : &str) -> String {
    aoclib::knot_hash::knot_hash_as_hex(input)
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
}
