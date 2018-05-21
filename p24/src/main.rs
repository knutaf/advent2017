#![feature(nll)]

use std::fmt;
use std::collections::HashSet;
use std::hash::{Hash, Hasher};

extern crate aoclib;
use aoclib::*;

#[derive(Clone)]
struct Component {
    num : usize,
    port1 : u32,
    port2 : u32,
}

#[derive(Clone)]
struct Bridge<'p> {
    unused : HashSet<&'p Component>,
    used : Vec<&'p Component>,
    last_port : u32,
}

impl Component {
    fn parse(line : &str, num : usize) -> Component {
        let mut split = line.split('/');
        Component {
            num,
            port1 : split.next().unwrap().parse::<u32>().unwrap(),
            port2 : split.next().unwrap().parse::<u32>().unwrap(),
        }
    }

    fn get_strength(&self) -> u32 {
        self.port1 + self.port2
    }
}

impl PartialEq for Component {
    fn eq(&self, other : &Component) -> bool {
        self.num == other.num
    }
}

impl Hash for Component {
    fn hash<H : Hasher>(&self, state : &mut H) {
        self.num.hash(state);
    }
}

impl Eq for Component { }

impl fmt::Display for Component {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{} {}/{}]", self.num, self.port1, self.port2)
    }
}

impl fmt::Debug for Component {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{} {}/{}]", self.num, self.port1, self.port2)
    }
}

impl<'p> Bridge<'p> {
    fn new(parts : &'p HashSet<Component>) -> Bridge<'p> {
        Bridge {
            unused : parts.iter().collect(),
            used : Vec::with_capacity(parts.len()),
            last_port : 0,
        }
    }

    fn split_with_part(&self) -> Vec<Bridge<'p>> {
        self.unused.iter().filter(|part| {
            part.port1 == self.last_port || part.port2 == self.last_port
        }).map(|compatible| {
            let mut bridge = self.clone();

            let dummy_thing = compatible.clone();
            bridge.unused.remove(&dummy_thing);

            if bridge.last_port == dummy_thing.port1 {
                bridge.last_port = dummy_thing.port2;
            } else {
                bridge.last_port = dummy_thing.port1;
            }

            bridge.used.push(compatible);

            bridge
        }).collect()
    }

    fn get_strength(&self) -> u32 {
        self.used.iter().fold(0, |sofar, item| {
            sofar + item.get_strength()
        })
    }

    fn get_length(&self) -> usize {
        self.used.len()
    }
}

impl<'p> fmt::Display for Bridge<'p> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut ret = write!(f, "");
        for part in self.used.iter() {
            ret = write!(f, "{}", part);
        }
        ret
    }
}

fn get_strongest_bridge_strength(input : &str) -> u32 {
    let components = input.lines().enumerate().map(|(i, line)| {
        Component::parse(line, i)
    }).collect::<HashSet<_>>();

    let mut frontier = vec![Bridge::new(&components)];
    let mut current_explore_index = 0;
    let mut strongest = 0;
    while current_explore_index < frontier.len() {
        //eprintln!("current: {}, last_port: {}, unused: {:?}", frontier[current_explore_index], frontier[current_explore_index].last_port, frontier[current_explore_index].unused);

        let current_strength = frontier[current_explore_index].get_strength();
        if current_strength > strongest {
            eprintln!("strongest upgraded to {} ({}). frontier size is {}", frontier[current_explore_index], current_strength, frontier.len());
            strongest = current_strength;
        }

        let len = frontier.len();
        let mut split = frontier[current_explore_index].split_with_part();
        frontier.append(&mut split);
        //eprintln!("{} added {} to explore. frontier size now {}", current_explore_index, frontier.len() - len, frontier.len());

        current_explore_index += 1;
        //eprintln!("moving on to explore {}", current_explore_index);
    }

    strongest
}

fn get_longest_bridge_strength(input : &str) -> u32 {
    let components = input.lines().enumerate().map(|(i, line)| {
        Component::parse(line, i)
    }).collect::<HashSet<_>>();

    let mut frontier = vec![Bridge::new(&components)];
    let mut current_explore_index = 0;
    let mut longest_length = 0;
    let mut longest_strength = 0;
    while current_explore_index < frontier.len() {
        //eprintln!("current: {}, last_port: {}, unused: {:?}", frontier[current_explore_index], frontier[current_explore_index].last_port, frontier[current_explore_index].unused);

        let current_length = frontier[current_explore_index].get_length();
        if current_length > longest_length {
            longest_length = current_length;
            longest_strength = frontier[current_explore_index].get_strength();
            eprintln!("longest upgraded to {} ({}/{}). frontier size is {}", frontier[current_explore_index], longest_length, longest_strength, frontier.len());
        } else if current_length == longest_length {
            let current_strength = frontier[current_explore_index].get_strength();
            if current_strength > longest_strength {
                longest_strength = current_strength;
                eprintln!("strongest upgraded to {} ({}/{}). frontier size is {}", frontier[current_explore_index], longest_length, longest_strength, frontier.len());
            }
        }

        let len = frontier.len();
        let mut split = frontier[current_explore_index].split_with_part();
        frontier.append(&mut split);
        //eprintln!("{} added {} to explore. frontier size now {}", current_explore_index, frontier.len() - len, frontier.len());

        current_explore_index += 1;
        //eprintln!("moving on to explore {}", current_explore_index);
    }

    longest_strength
}

fn solve_a(input : &str) -> u32 {
    get_strongest_bridge_strength(input)
}

fn solve_b(input : &str) -> u32 {
    get_longest_bridge_strength(input)
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
r"0/2
2/2
2/3
3/4
3/5
0/1
10/1
9/10";
        assert_eq!(solve_a(&input), 31);
    }

    #[test]
    fn a_1() {
        let input =
r"0/1000
0/100";
        assert_eq!(solve_a(&input), 1000);
    }

    #[test]
    fn a_2() {
        let input =
r"0/1000
0/100
100/901";
        assert_eq!(solve_a(&input), 1101);

        let input =
r"100/901
0/1000
0/100";
        assert_eq!(solve_a(&input), 1101);

        let input =
r"100/901
0/100
0/1000";
        assert_eq!(solve_a(&input), 1101);
    }

    #[test]
    fn a_combos() {
        let input =
r"0/1
1/2
1/4
2/3
1/5
2/5";
        assert_eq!(solve_a(&input), 22);
    }

    #[test]
    fn b_given() {
        let input =
r"0/2
2/2
2/3
3/4
3/5
0/1
10/1
9/10";
        assert_eq!(solve_b(&input), 19);
    }
}
