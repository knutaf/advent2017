extern crate aoclib;
use aoclib::*;

#[derive(Clone)]
struct State {
    mem : Vec<u32>,
}

impl State {
    fn select(&self) -> (usize, u32) {
        self.mem.iter().enumerate().fold((0, 0), |sel, (i, &val)| {
            if val > sel.1 {
                (i, val)
            } else {
                sel
            }
        })
    }

    fn redistribute(&self) -> State {
        let mut new_state = self.clone();
        let (selected_i, selected_val) = new_state.select();

        let len = new_state.mem.len();
        *(new_state.mem.get_mut(selected_i).unwrap()) = 0;
        for dist_i in 0 .. (selected_val as usize) {
            *(new_state.mem.get_mut((selected_i + 1 + dist_i) % len).unwrap()) += 1;
        }

        new_state
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

    fn sel_test(arr : &[u32], sel_i : usize) {
        let s = State {
            mem: arr.to_vec(),
        };

        assert_eq!(s.select().0, sel_i);
    }

    fn redist_test(before : &[u32], after : &[u32]) {
        let s = State {
            mem: before.to_vec(),
        };

        assert_eq!(s.redistribute().mem, after.to_vec());
    }

    #[test]
    fn sel_1() {
        sel_test(&[0, 1, 2], 2);
        sel_test(&[2, 1, 0], 0);
        sel_test(&[1, 2, 1], 1);
    }

    #[test]
    fn redist_1() {
        redist_test(&[0, 1, 2], &[1, 2, 0]);
        redist_test(&[2, 1, 0], &[0, 2, 1]);
        redist_test(&[2, 1], &[1, 2]);
    }

    #[test]
    fn redist_2() {
        redist_test(&[0, 2, 7, 0], &[2, 4, 1, 2]);
    }

    #[test]
    fn a_1() {
        let input = "0	2	7	0";
        assert_eq!(solve_a(&input), 5);
    }

    #[test]
    fn b_1() {
        let input = "blah";
        assert_eq!(solve_b(&input), 0);
    }
}
