extern crate aoclib;
use aoclib::*;

use std::f32;

#[derive(Debug, PartialEq, Clone)]
struct Pos {
    address : u32,
    x : i32,
    y : i32,
}

fn pos(address : u32, x : i32, y : i32) -> Pos {
    Pos { address, x, y }
}

impl Pos {
    fn spiral_iter(&self) -> SpiralWalker {
        SpiralWalker {
            pos: self.clone()
        }
    }
}

struct SpiralWalker {
    pos: Pos
}

impl Iterator for SpiralWalker {
    type Item = Pos;

    fn next(&mut self) -> Option<Self::Item> {
        let ring_start_pos = get_ring_start_position(self.pos.address);
        let ring_side_steps = get_inner_box_side(self.pos.address) + 1;

        let ring_base_addr = ring_start_pos.address - 1;

        let next_delta =
            if self.pos.address >= ring_base_addr + (ring_side_steps * 3) {
                (1, 0)
            } else if self.pos.address >= ring_base_addr + (ring_side_steps * 2) {
                (0, -1)
            } else if self.pos.address >= ring_base_addr + (ring_side_steps * 1) {
                (-1, 0)
            } else {
                (0, 1)
            };

        self.pos = Pos {
            address: self.pos.address + 1,
            x: self.pos.x + next_delta.0,
            y: self.pos.y + next_delta.1,
        };

        Some(self.pos.clone())
    }
}

fn get_inner_box_side(address : u32) -> u32 {
    let sqrt = ((address - 1) as f32).sqrt() as u32;
    if sqrt == 0 || sqrt % 2 != 0 {
        sqrt
    } else {
        sqrt - 1
    }
}

fn get_ring_start_position(address : u32) -> Pos {
    let inner_box_side = get_inner_box_side(address);

    // next spiral ring begins just to the right of the highest inner box position.
    Pos {
        address: (inner_box_side * inner_box_side) + 1,
        x: ((inner_box_side / 2) as i32) + 1,
        y: -((inner_box_side / 2) as i32),
    }
}

fn get_position_of_address(address : u32) -> Pos {
    let ring_start_pos = get_ring_start_position(address);

    if ring_start_pos.address == address {
        ring_start_pos
    } else {
        ring_start_pos.spiral_iter().find(|p| {
            p.address == address
        }).expect("couldn't find address")
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
    let p = get_position_of_address(address);
    (p.x.abs() + p.y.abs()) as u32
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
    fn box_side() {
        assert_eq!(get_inner_box_side(1), 0);
        assert_eq!(get_inner_box_side(2), 1);
        assert_eq!(get_inner_box_side(9), 1);
        assert_eq!(get_inner_box_side(8), 1);
        assert_eq!(get_inner_box_side(10), 3);
        assert_eq!(get_inner_box_side(25), 3);
        assert_eq!(get_inner_box_side(26), 5);
    }

    #[test]
    fn ring_start_pos() {
        assert_eq!(get_ring_start_position(2), pos(2, 1, 0));
        assert_eq!(get_ring_start_position(9), pos(2, 1, 0));
        assert_eq!(get_ring_start_position(10), pos(10, 2, -1));
        assert_eq!(get_ring_start_position(25), pos(10, 2, -1));
        assert_eq!(get_ring_start_position(26), pos(26, 3, -2));
    }

    fn addr_pos_test(address : u32) -> (i32, i32) {
        let p = get_position_of_address(address);
        assert_eq!(p.address, address);
        (p.x, p.y)
    }

    #[test]
    fn addr_pos_inner() {
        assert_eq!(addr_pos_test(2), (1, 0));
        assert_eq!(addr_pos_test(3), (1, 1));
        assert_eq!(addr_pos_test(4), (0, 1));
        assert_eq!(addr_pos_test(5), (-1, 1));
        assert_eq!(addr_pos_test(6), (-1, 0));
        assert_eq!(addr_pos_test(7), (-1, -1));
        assert_eq!(addr_pos_test(8), (0, -1));
        assert_eq!(addr_pos_test(9), (1, -1));
    }

    #[test]
    fn addr_pos_outer() {
        assert_eq!(addr_pos_test(10), (2, -1));
        assert_eq!(addr_pos_test(25), (2, -2));
        assert_eq!(addr_pos_test(17), (-2, 2));
    }

    #[test]
    fn a_1() {
        assert_eq!(solve_a("10"), 3);
        assert_eq!(solve_a("25"), 4);
        assert_eq!(solve_a("1024"), 31);
    }


    #[test]
    fn b_1() {
        let input = "0";
        assert_eq!(solve_b(&input), 0);
    }
}
