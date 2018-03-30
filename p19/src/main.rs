#![feature(nll)]

use std::fmt;

extern crate aoclib;
use aoclib::*;

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct RoutingTable {
    grid : Vec<Vec<char>>,
}

struct WalkerPosition {
    dir : Direction,
    x : usize,
    y : usize,
}

struct RouteWalker<'t> {
    table : &'t RoutingTable,
    position : WalkerPosition,
    collected_letters : String,
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Direction::Up => write!(f, "U"),
            &Direction::Down => write!(f, "D"),
            &Direction::Left => write!(f, "L"),
            &Direction::Right => write!(f, "R"),
        }
    }
}

impl fmt::Display for WalkerPosition {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}, x:{}, y:{}", self.dir, self.x, self.y)
    }
}

impl RoutingTable {
    fn load(input : &str) -> RoutingTable {
        RoutingTable {
            grid: input.lines().map(|line| {
                line.chars().collect()
            }).collect()
        }
    }

    fn walk<'t>(&'t self) -> RouteWalker<'t> {
        let starting_x = self.grid[0].iter().enumerate().find(|&(_, ch)| {
            *ch == '|'
        }).map(|(x, _)| {
            x
        }).unwrap();

        eprintln!("starting walk at ({}, 0)", starting_x);

        RouteWalker::new(self, (starting_x, 0))
    }
}

impl<'t> RouteWalker<'t> {
    fn new(table : &'t RoutingTable, initial_position : (usize, usize)) -> RouteWalker<'t> {
        RouteWalker {
            table : table,
            position : WalkerPosition { dir : Direction::Down, x : initial_position.0, y : initial_position.1 },
            collected_letters : String::new(),
        }
    }

    fn is_filled(&self, x : usize, y : usize) -> bool {
        match self.table.grid[y][x] {
            ' ' => false,
            _ => true,
        }
    }

    fn move_one_step(&mut self) {
        match self.position.dir {
            Direction::Up => {
                if self.position.y > 0 {
                    self.position.y -= 1;
                } else {
                    self.position.y = self.table.grid.len();
                }
            },
            Direction::Down => {
                self.position.y += 1;
            },
            Direction::Left => {
                if self.position.x > 0 {
                    self.position.x -= 1;
                } else {
                    self.position.x = self.table.grid[0].len();
                }
            },
            Direction::Right => {
                self.position.x += 1;
            },
        }
    }
}

impl<'t> Iterator for RouteWalker<'t> {
    type Item = &'t String;

    fn next(&mut self) -> Option<Self::Item> {
        //eprintln!("{}", self.position);

        if self.position.y < self.table.grid.len() &&
           self.position.x < self.table.grid[0].len() {
            match self.table.grid[self.position.y][self.position.x] {
                '|' | '-' => { },
                '+' => {
                    match self.position.dir {
                        Direction::Up | Direction::Down => {
                            if self.position.x > 0 && self.is_filled(self.position.x - 1, self.position.y) {
                                eprintln!("{} -> left", self.position);
                                self.position.dir = Direction::Left;
                            } else if self.position.x < self.table.grid[0].len() - 1 && self.is_filled(self.position.x + 1, self.position.y) {
                                eprintln!("{} -> right", self.position);
                                self.position.dir = Direction::Right;
                            }
                        },
                        Direction::Left | Direction::Right => {
                            if self.position.y > 0 && self.is_filled(self.position.x, self.position.y - 1) {
                                eprintln!("{} -> up", self.position);
                                self.position.dir = Direction::Up;
                            } else if self.position.y < self.table.grid.len() - 1 && self.is_filled(self.position.x, self.position.y + 1) {
                                eprintln!("{} -> down", self.position);
                                self.position.dir = Direction::Down;
                            }
                        },
                    }
                },
                ' ' => {
                    eprintln!("    Stepped on a blank. must be done");
                    self.position.x = self.table.grid[0].len();
                    self.position.y = self.table.grid.len();
                },
                ch => {
                    self.collected_letters.push(ch);
                    eprintln!("{}", self.position);
                    eprintln!("    grabbed {}. now {}", ch, self.collected_letters);
                }
            }

            self.move_one_step();

            Some(&self.collected_letters)
        } else {
            None
        }
    }
}

fn solve_a(input : &str) -> String {
    let routing_table = RoutingTable::load(input);
    routing_table.walk().last().unwrap().clone()
}

fn solve_b(input : &str) -> u32 {
    0
}

fn main() {
    let input = read_all_stdin_notrim();
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
    fn a_1() {
        let input =
r"     |          
     |  +--+    
     A  |  C    
 F---|----E|--+ 
     |  |  |  D 
     +B-+  +--+ ";
        assert_eq!(solve_a(&input), "ABCDEF");
    }

    #[test]
    fn b_1() {
        let input = "blah";
        assert_eq!(solve_b(&input), 0);
    }
}
