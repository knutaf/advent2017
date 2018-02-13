#![feature(nll)]

extern crate aoclib;
use aoclib::*;

struct Layer {
    depth : u32,
    range : u32,
}

struct Firewall {
    layers : Vec<Option<Layer>>,
}

struct PacketPasser<'t> {
    firewall : std::slice::Iter<'t, Option<Layer>>,
    t : u32,
    cost : u32,
}

impl Firewall {
    fn from(input : &str) -> Firewall {
        let mut layers = vec![];

        for line in input.lines() {
            let split : Vec<&str> = line.split(": ").collect();

            let layer = Layer {
                depth : split[0].parse::<u32>().unwrap(),
                range : split[1].parse::<u32>().unwrap(),
            };

            while layers.len() < (layer.depth as usize) {
                layers.push(None);
            }

            layers.push(Some(layer));
        }

        Firewall {
            layers : layers,
        }
    }

    fn simulate<'t>(&'t self) -> PacketPasser<'t> {
        PacketPasser {
            firewall : self.layers.iter(),
            t : 0,
            cost : 0,
        }
    }

    fn wrap_scanner(t : u32, range : u32) -> u32 {
        if range > 1 {
            // The scanner goes back and forth over the range// so unwrap to 2x the range. But the
            // endpoints don't quite count, so subtract them both.
            let unwrapped_range = (range * 2) - 2;
            let unwrapped_position = t % unwrapped_range;

            // If the scanner is within the range itself, just use that position. If it's in the
            // unwrapped part of the range, subtract the unwrapped range to get it back into the
            // regular range.
            if unwrapped_position < range {
                unwrapped_position
            } else {
                unwrapped_range - unwrapped_position
            }
        } else {
            0
        }
    }
}

impl<'t> Iterator for PacketPasser<'t> {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.firewall.next().map(|layer| {
            if let Some(layer) = layer.as_ref() {
                if Firewall::wrap_scanner(self.t, layer.range) == 0 {
                    self.cost += layer.depth * layer.range;
                }
            }

            self.t += 1;

            self.cost
        })
    }
}


fn solve_a(input : &str) -> u32 {
    let firewall = Firewall::from(input);
    firewall.simulate().inspect(|x| {eprintln!("{}", x);}).last().unwrap()
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
    fn wrap_scanner_3() {
       assert_eq!(Firewall::wrap_scanner(0, 3), 0);
       assert_eq!(Firewall::wrap_scanner(1, 3), 1);
       assert_eq!(Firewall::wrap_scanner(2, 3), 2);
       assert_eq!(Firewall::wrap_scanner(3, 3), 1);
       assert_eq!(Firewall::wrap_scanner(4, 3), 0);
       assert_eq!(Firewall::wrap_scanner(5, 3), 1);
       assert_eq!(Firewall::wrap_scanner(6, 3), 2);
       assert_eq!(Firewall::wrap_scanner(7, 3), 1);
       assert_eq!(Firewall::wrap_scanner(8, 3), 0);
    }

    #[test]
    fn wrap_scanner_6() {
       assert_eq!(Firewall::wrap_scanner(0, 6), 0);
       assert_eq!(Firewall::wrap_scanner(1, 6), 1);
       assert_eq!(Firewall::wrap_scanner(2, 6), 2);
       assert_eq!(Firewall::wrap_scanner(3, 6), 3);
       assert_eq!(Firewall::wrap_scanner(4, 6), 4);
       assert_eq!(Firewall::wrap_scanner(5, 6), 5);
       assert_eq!(Firewall::wrap_scanner(6, 6), 4);
       assert_eq!(Firewall::wrap_scanner(7, 6), 3);
       assert_eq!(Firewall::wrap_scanner(8, 6), 2);
       assert_eq!(Firewall::wrap_scanner(9, 6), 1);
       assert_eq!(Firewall::wrap_scanner(10, 6), 0);
       assert_eq!(Firewall::wrap_scanner(11, 6), 1);
       assert_eq!(Firewall::wrap_scanner(12, 6), 2);
    }

    #[test]
    fn a_given() {
        let input =
r"0: 3
1: 2
4: 4
6: 4";
        assert_eq!(solve_a(&input), 24);
    }

    #[test]
    fn a_all_caught() {
        let input =
r"0: 1
1: 1
2: 1
3: 1
4: 1
5: 1";
        assert_eq!(solve_a(&input), 15);
    }

    #[test]
    fn a_none_caught() {
        let input =
r"0: 1
1: 2
2: 3
3: 4
4: 5
5: 6";
        assert_eq!(solve_a(&input), 0);
    }

    #[test]
    fn b_1() {
        let input = "blah";
        assert_eq!(solve_b(&input), 0);
    }
}
