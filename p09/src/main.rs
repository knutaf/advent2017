extern crate aoclib;
use aoclib::*;

enum State {
    Normal,
    Garbage,
    GarbageCancel,
}

struct GroupCounter<'t> {
    chars : std::str::Chars<'t>,
    state : State,
    score : u32,
    depth : u32,
    garbage_count : u32,
}

impl<'t> GroupCounter<'t> {
    fn over(input : &'t str) -> GroupCounter<'t> {
        GroupCounter {
            chars : input.chars(),
            state : State::Normal,
            score : 0,
            depth : 0,
            garbage_count : 0,
        }
    }
}

impl<'t> Iterator for GroupCounter<'t> {
    type Item = (u32, u32);

    fn next(&mut self) -> Option<Self::Item> {
        self.chars.next().map(|ch| {
            match self.state {
                State::Normal => {
                    match ch {
                        '{' => {
                            self.depth += 1;
                        },
                        '}' => {
                            self.score += self.depth;
                            self.depth -= 1;
                        },
                        '<' => {
                            self.state = State::Garbage;
                        },
                        _ => {},
                    }
                },
                State::Garbage => {
                    match ch {
                        '>' => {
                            self.state = State::Normal;
                        },
                        '!' => {
                            self.state = State::GarbageCancel;
                        },
                        _ => {
                            self.garbage_count += 1;
                        },
                    }
                },
                State::GarbageCancel => {
                    self.state = State::Garbage;
                },
            }

            (self.score, self.garbage_count)
        })
    }
}

fn solve_a(input : &str) -> u32 {
    let counter = GroupCounter::over(input);
    counter.last().unwrap_or((0, 0)).0
}

fn solve_b(input : &str) -> u32 {
    let counter = GroupCounter::over(input);
    counter.last().unwrap_or((0, 0)).1
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
    fn a_1() {
        assert_eq!(solve_a("{}"), 1);
        assert_eq!(solve_a("{{{}}}"), 6);
        assert_eq!(solve_a("{{},{}}"), 5);
        assert_eq!(solve_a("{{{},{},{{}}}}"), 16);
        assert_eq!(solve_a("{<{},{},{{}}>}"), 1);
        assert_eq!(solve_a("{<a>,<a>,<a>,<a>}"), 1);
        assert_eq!(solve_a("{{<ab>},{<ab>},{<ab>},{<ab>}}"), 9);
        assert_eq!(solve_a("{{<!!>},{<!!>},{<!!>},{<!!>}}"), 9);
        assert_eq!(solve_a("{{<a!>},{<a!>},{<a!>},{<ab>}}"), 3);
    }

    #[test]
    fn b_1() {
        assert_eq!(solve_b("{<>}"), 0);
        assert_eq!(solve_b("{<random characters>}"), 17);
        assert_eq!(solve_b("{<<<<>}"), 3);
        assert_eq!(solve_b("{<{!>}>}"), 2);
        assert_eq!(solve_b("{<!!>}"), 0);
        assert_eq!(solve_b("{<!!!>>}"), 0);
        assert_eq!(solve_b("{<{o\"i!a,<{i<a>}"), 10);
    }
}
