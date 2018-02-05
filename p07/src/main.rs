#![feature(nll)]

use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt;

#[macro_use] extern crate lazy_static;
extern crate regex;
use regex::Regex;

extern crate aoclib;
use aoclib::*;

// Stores just the raw parsed data from the input, not directly linked to
// supported programs.
#[derive(PartialEq, Debug)]
struct ProgInfo<'t> {
    name : &'t str,
    weight : u32,
    child_names : Vec<&'t str>,
}

type RcRefProg<'t> = Rc<RefCell<Prog<'t>>>;

// Stores a program directly linked to its parent (if there is one) and
// supported programs.
struct Prog<'t> {
    name : &'t str,
    weight : u32,
    parent : Option<RcRefProg<'t>>,
    children : Vec<RcRefProg<'t>>,
}

struct FoundProg<'t> {
    prog : RcRefProg<'t>,
    depth : u32,
    weight_adjustment : i32,
}

type MaybeFoundProg<'t> = Option<FoundProg<'t>>;

impl<'t> FoundProg<'t> {
    fn new(prog : &RcRefProg<'t>, depth : u32, weight_adjustment : i32) -> MaybeFoundProg<'t> {
        Some(FoundProg {
            prog : Rc::clone(prog),
            depth : depth,
            weight_adjustment : weight_adjustment,
        })
    }
}

impl<'t> Prog<'t> {
    // Gets the weight of this program plus all children.
    fn get_subtree_weight(&self) -> u32 {
        self.children.iter().fold(self.weight, |sofar, child| {
            sofar + child.borrow().get_subtree_weight()
        })
    }

    // This function returns which child programs need to be investigated to see if they are the
    // single unbalanced program. It returns exactly 0, 1, or 2 child programs:
    // - 0 if there are no child programs OR if all child programs have the same subtree weight,
    //   then none need to be investigated because all children are balanced
    // - 1 if it's clear to identify which child program's subtree weight is the only unbalanced
    //   one, i.e. if it's a 2 v 1 situation.
    // - 2 if there are exactly two children with different weights. With the information supplied,
    //   we can't tell which one is correct, so both need to be investigated.
    fn find_unbalanced_subtrees(&self, depth : u32, previous_weight_adjustment : i32) -> (MaybeFoundProg<'t>, MaybeFoundProg<'t>) {
        // Get the first child's subtree weight, or just use 0 if there are
        // no children.
        let weight_0 = self.children.get(0).map_or(0, |child| child.borrow().get_subtree_weight());

        // Partition into two sets, based on whether equal to the first child's subtree
        // weight (partition.0) or not (partition.1).
        let partition : (Vec<&RcRefProg<'t>>, Vec<&RcRefProg<'t>>) = self.children.iter().partition(|child| {
            child.borrow().get_subtree_weight() == weight_0
        });

        match partition.0.len() {
            0 => {
                match partition.1.len() {
                    // 0 and 0. There are no children, so nothing to be investigated
                    0 => {
                        (None, None)
                    },
                    _ => {
                        // 0 and anything
                        panic!("impossible: can't have subtree weights unequal to the first child's subtree weight if none equal to the first child's subtree weight");
                    },
                }
            },
            1 => {
                match partition.1.len() {
                    0 => {
                        // 1 and 0. There's only one child, so we know which one needs to be
                        // investigated. With only one child program we can't figure out what the
                        // weight adjustment in that subtree should be, so pass on whatever was
                        // supplied to us.
                        (FoundProg::new(partition.0[0], depth, previous_weight_adjustment), None)
                    },
                    1 => {
                        // 1 and 1
                        // There are exactly two child programs with different weights. We need to
                        // investigate both of them. Also supply the weight adjustment for each as
                        // the difference between subtree weights (i.e. the weight needed to bring
                        // a subtree to be the same weight as the other one).
                        let weight_1 = partition.1[0].borrow().get_subtree_weight();
                        let weight_adjustment = (weight_1 as i32) - (weight_0 as i32);
                        (FoundProg::new(partition.0[0], depth, weight_adjustment), FoundProg::new(partition.1[0], depth, -weight_adjustment))
                    },
                    _ => {
                        // It's a 2 vs 1 weight situation, so we know the one with only 1 child
                        // program in the partition is the only one we need to investigate.
                        (FoundProg::new(partition.0[0], depth, (partition.1[0].borrow().get_subtree_weight() as i32) - (partition.0[0].borrow().get_subtree_weight() as i32)), None)
                    }
                }
            },
            _ => {
                match partition.1.len() {
                    0 => {
                        // 2 and 0. All child programs' subtree weights are the same, so nothing
                        // to investigate.
                        (None, None)
                    },
                    1 => {
                        // 2 and 1. We know the one alone is the only one we need to investigate.
                        (FoundProg::new(partition.1[0], depth, (partition.0[0].borrow().get_subtree_weight() as i32) - (partition.1[0].borrow().get_subtree_weight() as i32)), None)
                    },
                    _ => {
                        panic!("invalid input - should never have two subtrees that are unbalanced");
                    }
                }
            },
        }
    }
}

impl<'t> fmt::Display for Prog<'t> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let _ = write!(f, "[{} ({})", self.name, self.weight);
        for (i, c) in self.children.iter().enumerate() {
            if i == 0 {
                let _ = write!(f, " -> {}", c.borrow().name);
            } else {
                let _ = write!(f, ", {}", c.borrow().name);
            }
        }

        write!(f, "]")
    }
}

struct ProgDb<'t> {
    db : HashMap<&'t str, RcRefProg<'t>>,
}

impl<'t> ProgDb<'t> {
    fn new() -> ProgDb<'t> {
        ProgDb {
            db : HashMap::<&'t str, RcRefProg<'t>>::new(),
        }
    }

    fn from_input(input : &'t str) -> ProgDb<'t> {
        let mut pdb = ProgDb::new();

        for line in input.lines() {
            pdb.load_prog(parse_prog(line))
        }

        pdb
    }

    // Take a ProgInfo and add it to the map, either creating a new object or
    // filling in the details of a skeleton object from a previous load_prog
    // call. Link it to any children, too. If the children are already in the
    // map, link to the already existing objects. If not, create new skeleton
    // objects that will be filled in later by another load_prog call.
    fn load_prog(&mut self, prog_info : ProgInfo<'t>) {
        let new_prog = match self.db.get(prog_info.name) {
            None => Rc::new(RefCell::new(Prog {
                name : prog_info.name,
                weight : prog_info.weight,
                parent : None,
                children : vec![],
            })),
            Some(prog) => prog.clone(),
        };

        new_prog.borrow_mut().weight = prog_info.weight;

        for child_name in prog_info.child_names {
            let child_prog = match self.db.get(child_name) {
                None => {
                    let new_child = Rc::new(RefCell::new(Prog {
                        name : child_name,
                        weight : 0,
                        parent : Some(new_prog.clone()),
                        children : vec![],
                    }));

                    self.db.insert(child_name, new_child.clone());

                    new_child
                },
                Some(cp) => {
                    let mut existing_child_prog = cp.clone();
                    cp.borrow_mut().parent = Some(new_prog.clone());
                    existing_child_prog
                }
            };

            new_prog.borrow_mut().children.push(child_prog);
        }

        self.db.insert(prog_info.name, new_prog);
    }

    fn get(&self, prog_name : &str) -> Option<RcRefProg<'t>> {
        self.db.get(prog_name).map(|v| {
            v.clone()
        })
    }

    // Search for any program that doesn't have a parent link. That one is the
    // root.
    fn get_root(&self) -> RcRefProg<'t> {
        self.db.values().find(|prog| {
            prog.borrow().parent.is_none()
        }).unwrap().clone()
    }
}

fn parse_prog<'t>(line : &'t str) -> ProgInfo<'t> {
    lazy_static! {
        static ref RE_PROG_INFO : regex::Regex = Regex::new(r"^(\w+) \((\d+)\)").expect("failed to compile regex");
        static ref RE_PROG_CHILDREN : regex::Regex = Regex::new(r",? (\w+)").expect("failed to compile regex");
    }

    let prog_info_captures = RE_PROG_INFO.captures_iter(line).nth(0).unwrap();

    let children_captures_iter = RE_PROG_CHILDREN.captures_iter(line);

    ProgInfo {
        name : prog_info_captures.get(1).unwrap().as_str(),
        weight : prog_info_captures[2].parse::<u32>().unwrap(),
        child_names : children_captures_iter.map(|caps| {
            caps.get(1).unwrap().as_str()
        }).collect(),
    }
}

fn solve_a<'t>(input : &'t str) -> &'t str {
    let db = ProgDb::from_input(input);
    db.get_root().borrow().name
}

// Helper function that takes two candidates that are unbalanced and picks the one that came from
// deepest in the tree.
fn pick_deepest_program<'t>(candidate0 : MaybeFoundProg<'t>, candidate1 : MaybeFoundProg<'t>) -> MaybeFoundProg<'t> {
    if let Some(c0) = candidate0 {
        if let Some(c1) = candidate1 {
            if c0.depth > c1.depth {
                Some(c0)
            } else {
                Some(c1)
            }
        } else {
            Some(c0)
        }
    } else if candidate1.is_some() {
        candidate1
    } else {
        None
    }
}

// Searches for an unbalanced program under this one. It only picks from a child program or
// something underneath it, not this program itself. It is already supplied the weight adjustment
// needed to balance the whole tree, and it keeps track of how deep in the traversal it is.
fn find_unbalanced_child_program<'t>(program : &RcRefProg<'t>, current_depth : u32, weight_adjustment : i32) -> MaybeFoundProg<'t> {
    // Ask for which child programs need to be investigated, and what weight adjustment is needed
    // to balance each one.
    let (subtree0, subtree1) = program.borrow().find_unbalanced_subtrees(current_depth, weight_adjustment);

    eprintln!("find_unbalanced_child_program {}, depth {}, adj {}, trees ({}, {})", program.borrow(), current_depth, weight_adjustment, subtree0.is_some(), subtree1.is_some());

    // Helper function to search a subtree only if the weight adjustment would balance the tree if
    // applied to it.
    let find_unbalanced_program_in_subtree = |subtree : FoundProg<'t>| {
        eprintln!("find_unbalanced_program_in_subtree {}, adj {}", subtree.prog.borrow(), subtree.weight_adjustment);
        if current_depth == 0 || subtree.weight_adjustment == weight_adjustment {
            // If the weight adjustment would fix this subtree, then either we pick something
            // deeper in the subtree or that program itself.
            find_unbalanced_child_program(&subtree.prog, subtree.depth + 1, subtree.weight_adjustment).or(Some(subtree))
        } else {
            None
        }
    };

    // Search both sides of the partition and pick the deepest program that fixes the balance of
    // the tree, in the case of having exactly two children.
    let candidate0 = subtree0.and_then(&find_unbalanced_program_in_subtree);
    let candidate1 = subtree1.and_then(&find_unbalanced_program_in_subtree);
    pick_deepest_program(candidate0, candidate1)
}

fn solve_b_with_program<'t>(input : &'t str) -> (MaybeFoundProg<'t>, u32) {
    let db = ProgDb::from_input(input);
    let root = db.get_root();

    find_unbalanced_child_program(&root, 0, 0).map_or_else(|| {
        panic!("failed to find unbalanced program");
    },
    |found_program| {
        let weight = ((found_program.prog.borrow().weight as i32) + found_program.weight_adjustment) as u32;
        eprintln!("Found unbalanced program as {} at depth {}, with weight adjustment {}", found_program.prog.borrow(), found_program.depth, found_program.weight_adjustment);
        (Some(found_program), weight)
    })
}

fn solve_b(input : &str) -> u32 {
    solve_b_with_program(input).1
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
    fn parse_no_children() {
        let input = "pbga (66)";
        assert_eq!(parse_prog(&input), ProgInfo {
            name: "pbga",
            weight: 66,
            child_names: vec![],
        });
    }

    #[test]
    fn parse_with_children() {
        let input = "fwft (72) -> ktlj, cntj, xhth";
        assert_eq!(parse_prog(&input), ProgInfo {
            name: "fwft",
            weight: 72,
            child_names: vec!["ktlj", "cntj", "xhth"],
        });
    }

    #[test]
    fn weights_in_order() {
        let input =
r"b (1)
c (2)
d (3)
a (100) -> b, c, d";
        let db = ProgDb::from_input(input);
        assert_eq!(db.get("a").unwrap().borrow().weight, 100);
        assert_eq!(db.get("b").unwrap().borrow().weight, 1);
        assert_eq!(db.get("c").unwrap().borrow().weight, 2);
        assert_eq!(db.get("d").unwrap().borrow().weight, 3);
    }

    #[test]
    fn weights_backwards() {
        let input =
r"a (100) -> b, c, d
b (1)
c (2)
d (3)";
        let db = ProgDb::from_input(input);
        assert_eq!(db.get("a").unwrap().borrow().weight, 100);
        assert_eq!(db.get("b").unwrap().borrow().weight, 1);
        assert_eq!(db.get("c").unwrap().borrow().weight, 2);
        assert_eq!(db.get("d").unwrap().borrow().weight, 3);
    }


    #[test]
    fn subtree_weight_1() {
        let input =
r"a (100) -> b, c, d
b (1)
c (2)
d (3)";
        let db = ProgDb::from_input(input);
        assert_eq!(db.get("a").unwrap().borrow().get_subtree_weight(), 106);
        assert_eq!(db.get("b").unwrap().borrow().get_subtree_weight(), 1);
        assert_eq!(db.get("c").unwrap().borrow().get_subtree_weight(), 2);
        assert_eq!(db.get("d").unwrap().borrow().get_subtree_weight(), 3);
    }

    #[test]
    fn subtree_weight_2() {
        let input =
r"a (1) -> b
b (2) -> c
c (3) -> d
d (4) -> e
e (5) -> f
f (6)";
        let db = ProgDb::from_input(input);
        assert_eq!(db.get("a").unwrap().borrow().get_subtree_weight(), 21);
        assert_eq!(db.get("b").unwrap().borrow().get_subtree_weight(), 20);
        assert_eq!(db.get("f").unwrap().borrow().get_subtree_weight(), 6);
    }

    #[test]
    fn a_1() {
        let input =
r"a (100) -> b, c, d
b (1)
c (2)
d (3)";
        assert_eq!(solve_a(input), "a");
    }

    #[test]
    fn a_2() {
        let input =
r"b (1)
c (2)
d (3)
a (100) -> b, c, d";
        assert_eq!(solve_a(input), "a");
    }

    #[test]
    fn a_3() {
        let input =
r"f (6)
e (5) -> f
d (4) -> e
c (3) -> d
b (2) -> c
a (1) -> b";
        assert_eq!(solve_a(input), "a");
    }

    #[test]
    fn a_4() {
        let input =
r"a (1) -> b
b (2) -> c
c (3) -> d
d (4) -> e
e (5) -> f
f (6)";
        assert_eq!(solve_a(input), "a");
    }

    fn solve_b_test(input : &str, expected_program_name : &str, expected_weight : u32) {
        let (found_prog, weight) = solve_b_with_program(input);
        assert_eq!(found_prog.unwrap().prog.borrow().name, expected_program_name);
        assert_eq!(weight, expected_weight);
    }

    #[test]
    fn b_3_children_1() {
        //    1
        // 2  1  1
        let input =
r"a (1) -> aa, ab, ac
aa (2)
ab (1)
ac (1)";
        solve_b_test(&input, "aa", 1);
    }

    #[test]
    fn b_3_children_2() {
        //    1
        // 1  2  1
        let input =
r"a (1) -> aa, ab, ac
aa (1)
ab (2)
ac (1)";
        solve_b_test(&input, "ab", 1);
    }

    #[test]
    fn b_3_children_3() {
        //    1
        // 1  1  2
        let input =
r"a (1) -> aa, ab, ac
aa (1)
ab (1)
ac (2)";
        solve_b_test(&input, "ac", 1);
    }

    #[test]
    fn b_2_1() {
        //       1
        //   1       3
        // 1   2
        let input =
r"a (1) -> aa, ab
aa (1) -> aaa, aab
ab (3)
aaa (1)
aab (2)";
        solve_b_test(&input, "aab", 1);
    }

    #[test]
    fn b_2_2() {
        //       1
        //   1       3
        // 2   1
        let input =
r"a (1) -> aa, ab
aa (1) -> aaa, aab
ab (3)
aaa (2)
aab (1)";
        solve_b_test(&input, "aaa", 1);
    }

    #[test]
    fn b_2_3() {
        //    1
        // 3     1
        //     1   2
        let input =
r"a (1) -> aa, ab
aa (3)
ab (1) -> aba, abb
aba (1)
abb (2)";
        solve_b_test(&input, "abb", 1);
    }

    #[test]
    fn b_3() {
        let input =
r"a (1) -> b, c
b (1) -> d, e
c (5)
d (2)
e (1)";
        solve_b_test(&input, "e", 2);
    }

    #[test]
    fn b_4() {
        //     1
        //  1     1
        // 1 1   2 1
        let input =
r"a (1) -> aa, ab
aa (1) -> aaa, aab
aaa (1)
aab (1)
ab (1) -> aba, abb
aba (2)
abb (1)";
        solve_b_test(&input, "aba", 1);
    }

    #[test]
    fn b_5() {
        //     1
        //  1      1
        // 3 3   2   3
        //      1 1
        let input =
r"a (1) -> aa, ab
aa (1) -> aaa, aab
aaa (3)
aab (3)
ab (1) -> aba, abb
aba (2) -> abaa, abab
abaa (1)
abab (1)
abb (3)";
        solve_b_test(&input, "aba", 1);
    }

    #[test]
    fn b_6() {
        //     1
        //  1      1     7
        // 3 3   2   3
        //      1 1
        let input =
r"a (1) -> aa, ab, ac
aa (1) -> aaa, aab
aaa (3)
aab (3)
ab (1) -> aba, abb
aba (2) -> abaa, abab
abaa (1)
abab (1)
abb (3)
ac (7)";
        solve_b_test(&input, "aba", 1);
    }

    #[test]
    fn b_7() {
        //     1
        //  1      1     6
        // 3 3   1   3
        //      1 1
        let input =
r"a (1) -> aa, ab, ac
aa (1) -> aaa, aab
aaa (3)
aab (3)
ab (1) -> aba, abb
aba (1) -> abaa, abab
abaa (1)
abab (1)
abb (3)
ac (6)";
        solve_b_test(&input, "ac", 7);
    }

    #[test]
    fn b_8() {
        //     1
        //  1      1     8
        // 3 3   1   3
        //      1 1
        let input =
r"a (1) -> aa, ab, ac
aa (1) -> aaa, aab
aaa (3)
aab (3)
ab (1) -> aba, abb
aba (1) -> abaa, abab
abaa (1)
abab (1)
abb (3)
ac (8)";
        solve_b_test(&input, "ac", 7);
    }

    #[test]
    fn b_k01() {
        let input =
r"a (1) -> aa, ab, ac
aa (2)
ab (2)
ac (1)";
        solve_b_test(&input, "ac", 2);
    }

    #[test]
    fn b_k04() {
        let input =
r"a (1) -> aa, ab
aa (1) -> aaa, aab
ab (5)
aaa (1)
aab (2)";
        solve_b_test(&input, "aaa", 2);
    }

    #[test]
    fn b_k07() {
        let input =
r"a (1) -> aa, ab
aa (5) -> aaa, aab
aaa (1)
aab (1)
ab (1) -> aba
aba (1) -> abaa
abaa (1) -> abaaa, abaab
abaaa (2)
abaab (1)";
        solve_b_test(&input, "abaab", 2);
    }

    #[test]
    fn b_k08() {
        let input =
r"a (1) -> aa, ab, ac
aa (1) -> aaa, aab
aaa (3)
aab (3)
ab (1) -> aba, abb
aba (3) -> abaa, abab
abaa (1)
abab (1)
abb (3) -> abba, abbb
abba (1)
abbb (1)
ac (11)";
        solve_b_test(&input, "aa", 5);
    }
}
