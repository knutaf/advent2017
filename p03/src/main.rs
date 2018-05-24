extern crate aoclib;
use aoclib::*;

use std::f32;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Pos {
    address : u32,
    x : i32,
    y : i32,
}

fn pos(address : u32, x : i32, y : i32) -> Pos {
    Pos { address, x, y }
}

impl Pos {
    fn coords(&self) -> (i32, i32) {
        (self.x, self.y)
    }

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

    #[allow(identity_op)]
    fn next(&mut self) -> Option<Self::Item> {
        let next_delta =
            if self.pos.address == 1 {
                (1, 0)
            } else {
                let ring_start_pos = get_ring_start_position(self.pos.address);
                let ring_side_steps = get_inner_box_side(self.pos.address) + 1;

                let ring_base_addr = ring_start_pos.address - 1;

                if self.pos.address >= ring_base_addr + (ring_side_steps * 3) {
                    (1, 0)
                } else if self.pos.address >= ring_base_addr + (ring_side_steps * 2) {
                    (0, -1)
                } else if self.pos.address >= ring_base_addr + (ring_side_steps * 1) {
                    (-1, 0)
                } else {
                    (0, 1)
                }
            };

        self.pos = Pos {
            address: self.pos.address + 1,
            x: self.pos.x + next_delta.0,
            y: self.pos.y + next_delta.1,
        };

        Some(self.pos.clone())
    }
}

struct MemoryFiller {
    mem : HashMap<(i32, i32), u32>,
    walker : SpiralWalker,
    max_x : i32,
}

impl MemoryFiller {
    fn new() -> MemoryFiller {
        let ret = MemoryFiller {
            mem : HashMap::<(i32, i32), u32>::new(),
            walker : pos(1, 0, 0).spiral_iter(),
            max_x : 0,
        };

        let init_coords = ret.walker.pos.coords();
        let mut ret = ret;
        ret.set(&init_coords, 1);
        ret
    }

    fn get(&self, p : &(i32, i32)) -> &u32 {
        self.mem.get(p).unwrap_or(&0)
    }

    fn set(&mut self, p : &(i32, i32), v : u32) {
        let _ = self.mem.insert(p.clone(), v);
    }

    fn purge_from_ring_start(&mut self, p : &(i32, i32)) {
        assert!(p.0 > 0);
        self.mem.retain(|&(x, _), _| {
            x.abs() < p.0
        });
    }
}

impl Iterator for MemoryFiller {
    type Item = (Pos, u32);

    #[allow(identity_op)]
    fn next(&mut self) -> Option<Self::Item> {
        let next_pos = self.walker.next().unwrap();
        let next_coords = next_pos.coords();

        if next_coords.0 > self.max_x {
            self.max_x = next_coords.0;
            self.purge_from_ring_start(&next_coords);
        }

        let next_val =
            self.get(&(next_coords.0 + 1, next_coords.1 + 0)) +
            self.get(&(next_coords.0 + 1, next_coords.1 + 1)) +
            self.get(&(next_coords.0 + 0, next_coords.1 + 1)) +
            self.get(&(next_coords.0 - 1, next_coords.1 + 1)) +
            self.get(&(next_coords.0 - 1, next_coords.1 - 0)) +
            self.get(&(next_coords.0 - 1, next_coords.1 - 1)) +
            self.get(&(next_coords.0 - 0, next_coords.1 - 1)) +
            self.get(&(next_coords.0 + 1, next_coords.1 - 1));

        self.set(&next_coords, next_val);

        Some((next_pos, next_val))
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
    let target_value = aoclib::parse_nums::<u32>(&input).nth(0).expect("failed to parse input");
    let mut mem = MemoryFiller::new();
    mem.find(|&(ref _pos, stored_value)| {
        stored_value > target_value
    }).unwrap().1
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

    fn check_next_filler_step(mem : &mut MemoryFiller, expected_value : u32) {
        let (_pos, actual_value) = mem.next().unwrap();
        assert_eq!(actual_value, expected_value);
    }

    // 147  142  133  122   59
    // 304    5    4    2   57
    // 330   10    1    1   54
    // 351   11   23   25   26
    // 362  747  806  880  931
    #[test]
    fn memory_filler_iter() {
        let mut mem = MemoryFiller::new();
        check_next_filler_step(&mut mem, 1);
        check_next_filler_step(&mut mem, 2);
        check_next_filler_step(&mut mem, 4);
        check_next_filler_step(&mut mem, 5);
        check_next_filler_step(&mut mem, 10);
        check_next_filler_step(&mut mem, 11);
        check_next_filler_step(&mut mem, 23);
        check_next_filler_step(&mut mem, 25);
        check_next_filler_step(&mut mem, 26);
        check_next_filler_step(&mut mem, 54);
        check_next_filler_step(&mut mem, 57);
        check_next_filler_step(&mut mem, 59);
        check_next_filler_step(&mut mem, 122);
        check_next_filler_step(&mut mem, 133);
        check_next_filler_step(&mut mem, 142);
        check_next_filler_step(&mut mem, 147);
        check_next_filler_step(&mut mem, 304);
        check_next_filler_step(&mut mem, 330);
        check_next_filler_step(&mut mem, 351);
        check_next_filler_step(&mut mem, 362);
        check_next_filler_step(&mut mem, 747);
        check_next_filler_step(&mut mem, 806);
        check_next_filler_step(&mut mem, 880);
        check_next_filler_step(&mut mem, 931);
    }


    #[test]
    fn b_1() {
        assert_eq!(solve_b("1"), 2);
        assert_eq!(solve_b("2"), 4);
        assert_eq!(solve_b("142"), 147);
        assert_eq!(solve_b("146"), 147);
        assert_eq!(solve_b("880"), 931);
    }
}
