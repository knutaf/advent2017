extern crate aoclib;
use aoclib::*;

use std::f32;

fn get_inner_box_side(address : u32) -> u32 {
    let sqrt = ((address - 1) as f32).sqrt() as u32;
    if sqrt % 2 == 0 {
        sqrt - 1
    } else {
        sqrt
    }
}

// 37 36  35  34  33  32 31
// 38 17  16  15  14  13 30
// 39 18   5   4   3  12 29
// 40 19   6   1   2  11 28
// 41 20   7   8   9  10 27
// 42 21  22  23  24  25 26
// 43 44  45  46  47  48 49

fn solve_a(input : &str) -> u32 {
    let address = aoclib::parse_nums::<u32>(&input).nth(0).expect("failed to parse input");
    let inner_box_side = get_inner_box_side(address);
    eprintln!("inner box: {}", inner_box_side);
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

    #[test]
    fn box_side_1() {
        assert_eq!(get_inner_box_side(2), 1);
    }

    #[test]
    fn box_side_2() {
        assert_eq!(get_inner_box_side(9), 1);
    }

    #[test]
    fn box_side_3() {
        assert_eq!(get_inner_box_side(8), 1);
    }

    #[test]
    fn box_side_4() {
        assert_eq!(get_inner_box_side(10), 3);
    }

    #[test]
    fn box_side_5() {
        assert_eq!(get_inner_box_side(25), 3);
    }

    #[test]
    fn box_side_6() {
        assert_eq!(get_inner_box_side(26), 5);
    }

    #[test]
    fn a_1() {
        assert_eq!(0, 0);
    }

    #[test]
    fn b_1() {
        let input = "0";
        assert_eq!(solve_b(&input), 0);
    }
}
