#![feature(nll)]

use std::rc::Rc;
use std::cell::RefCell;
use std::cell::Ref;
use std::collections::HashMap;

#[macro_use] extern crate lazy_static;
extern crate regex;
use regex::Regex;

extern crate aoclib;
use aoclib::*;

#[derive(PartialEq, Debug)]
struct ProgInfo<'t> {
    name : &'t str,
    weight : u32,
    child_names : Vec<&'t str>,
}

type RcRefProg<'t> = Rc<RefCell<Prog<'t>>>;

struct Prog<'t> {
    name : &'t str,
    weight : u32,
    parent : Option<RcRefProg<'t>>,
    children : Vec<RcRefProg<'t>>,
}

impl<'t> Prog<'t> {
    fn get_subtree_weight(&self) -> u32 {
        self.children.iter().fold(self.weight, |sofar, child| {
            sofar + child.borrow().get_subtree_weight()
        })
    }

    fn get_balance_partition(&self) -> (Vec<RcRefProg<'t>, Vec<RcRefProg<'t>) {
        if self.children.is_empty() {
            (vec![], vec![])
        } else {
            let weight_0 = self.children[0].borrow().get_subtree_weight();
            self.children.iter().partition(|child| {
                child.borrow().get_subtree_weight() == weight_0
            })
        }
    }

    fn is_balanced(partition : &(Vec<RcRefProg<'t>, Vec<RcRefProg<'t>)) -> bool {
        !partition.0.is_empty() && partition.1.is_empty()
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

    fn get(&self, prog_name : &str) -> Option<Ref<Prog<'t>>> {
        self.db.get(prog_name).map(|v| {
            v.borrow()
        })
    }

    fn get_root(&self) -> Ref<Prog<'t>> {
        self.db.values().find(|prog| {
            prog.borrow().parent.is_none()
        }).unwrap().borrow()
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

    // seems like a bug. why can't I just return the value? it doesn't compile
    let ans = db.get_root().name;
    ans
}

fn find_unbalanced_node<'t>(node : Ref<Prog<'t>>, weight_adjustment : i32) -> Option<Ref<Prog<'t>>> {
    let partition = node.get_balance_partition();

    if Prog::is_balanced(&partition) {
        None
    } else {
        // child node
        if partition.0.is_empty() {
            None
        } else {
            if partition.1.is_empty() {
                panic!("partition 1 had length {}", partition.1.len());
            }

            let unbalanced_node =
                if partition.0.len() == 1 {
                    let maybe_unbalanced_node = find_unbalanced_node(partition.0[0].borrow(), weight_adjustment);
                    if maybe_unbalanced_node.is_some() {
                        maybe_unbalanced_node
                    } else if partition.0[0].borrow().get_subtree_weight() + weight_adjustment == partition.1[0].borrow().get_subtree_weight() {
                        Some(partition.0[0].borrow())
                    } else {
                        None
                    }
                } else {
                    None
                };

            let unbalanced_node =
                if unbalanced_node.is_some() {
                    unbalanced_node
                } else if partition.1.len() == 1 {
                    let maybe_unbalanced_node = find_unbalanced_node(partition.1[0].borrow(), weight_adjustment);
                    if maybe_unbalanced_node.is_some() {
                        maybe_unbalanced_node
                    } else if partition.1[0].borrow().get_subtree_weight() - weight_adjustment == partition.0[0].borrow().get_subtree_weight() {
                        Some(partition.1[0].borrow())
                    } else {
                        None
                    }
                } else {
                    None
                };

            unbalanced_node
        }
    }
}

fn solve_b(input : &str) -> u32 {
/*
    1
 1     1
1 1   2 1

    1
 1      1
2 2   2   1
     1 1
*/

    let db = ProgDb::from_input(input);
    let root = db.get_root();

    let partition = root.get_balance_partition();
    let weight_adjustment = (partition.1[0].borrow().get_subtree_weight() as i32) - (partition.0[0].borrow().get_subtree_weight() as i32);

    if let Some(unbalanced_node) = find_unbalanced_node(root, weight_adjustment) {
        unbalanced_node.weight + weight_adjustment
    } else {
        panic!("failed to find unbalanced node");
    }
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
        assert_eq!(db.get("a").unwrap().weight, 100);
        assert_eq!(db.get("b").unwrap().weight, 1);
        assert_eq!(db.get("c").unwrap().weight, 2);
        assert_eq!(db.get("d").unwrap().weight, 3);
    }

    #[test]
    fn weights_backwards() {
        let input =
r"a (100) -> b, c, d
b (1)
c (2)
d (3)";
        let db = ProgDb::from_input(input);
        assert_eq!(db.get("a").unwrap().weight, 100);
        assert_eq!(db.get("b").unwrap().weight, 1);
        assert_eq!(db.get("c").unwrap().weight, 2);
        assert_eq!(db.get("d").unwrap().weight, 3);
    }


    #[test]
    fn subtree_weight_1() {
        let input =
r"a (100) -> b, c, d
b (1)
c (2)
d (3)";
        let db = ProgDb::from_input(input);
        assert_eq!(db.get("a").unwrap().get_subtree_weight(), 106);
        assert_eq!(db.get("b").unwrap().get_subtree_weight(), 1);
        assert_eq!(db.get("c").unwrap().get_subtree_weight(), 2);
        assert_eq!(db.get("d").unwrap().get_subtree_weight(), 3);
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
        assert_eq!(db.get("a").unwrap().get_subtree_weight(), 21);
        assert_eq!(db.get("b").unwrap().get_subtree_weight(), 20);
        assert_eq!(db.get("f").unwrap().get_subtree_weight(), 6);
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

    #[test]
    fn b_1() {
        let input =
r"a (1) -> b, c, d
b (1)
c (1)
d (2)";
        assert_eq!(solve_b(&input), 1);
    }

    #[test]
    fn b_2() {
        let input =
r"a (1) -> b, c
b (1) -> d, e
c (3)
d (1)
e (2)";
        assert_eq!(solve_b(&input), 1);
    }

    #[test]
    fn b_3() {
        let input =
r"a (1) -> b, c
b (1) -> d, e
c (5)
d (2)
e (1)";
        assert_eq!(solve_b(&input), 2);
    }

    #[test]
    fn b_4() {
        let input =
r"a (1) -> aa, ab
aa (1) -> aaa, aab
aaa (1)
aab (1)
ab (1) -> aba, abb
aba (2)
abb (1)";
        assert_eq!(solve_b(&input), 1);
    }

    #[test]
    fn b_5() {
        let input =
r"a (1) -> aa, ab
aa (1) -> aaa, aab
aaa (2)
aab (2)
ab (1) -> aba, abb
aba (2) -> abaa, abab
abaa (1)
abab (1)
abb (1)";
        assert_eq!(solve_b(&input), 1);
    }
}
