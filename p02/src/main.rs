extern crate aoclib;
use aoclib::*;

fn solve_a(input : &str) -> u32 {
    input.lines().fold(0u32, |sum, line| {
        let (min, max) = aoclib::parse_nums::<u32>(&line)
            .fold((None, None), |(min_opt, max_opt), num| {
            (
                match min_opt {
                    None => Some(num),
                    Some(min) if num < min => Some(num),
                    _ => min_opt,
                },
                match max_opt {
                    None => Some(num),
                    Some(max) if num > max => Some(num),
                    _ => max_opt,
                },
            )
        });

        let min = min.expect("somehow got None min");
        let max = max.expect("somehow got None max");

        eprintln!("min: {}, max: {}", min, max);

        sum + (max - min)
    })
}

fn solve_b(input : &str) -> u32 {
    input.lines().fold(0u32, |sum, line| {
        let row_nums : Vec<u32> = aoclib::parse_nums::<u32>(&line).collect();
        let divided = row_nums.iter().enumerate().fold(None, |divided_opt, (i, &num1)| {
            match divided_opt {
                None => {
                    match row_nums.iter().skip(i + 1).find(|&num2| {
                        eprintln!("checking {} divisible by {}", num1, num2);
                        num1 % num2 == 0 || num2 % num1 == 0
                    }) {
                        None => None,
                        Some(num2) => if num1 % num2 == 0 {
                            Some(num1 / num2)
                        } else {
                            Some(num2 / num1)
                        },
                    }
                },
                _ => divided_opt,
            }
        }).expect("didn't find divided num");

        eprintln!("div: {}", divided);

        sum + divided
    })
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
        let input =
r"5 1 9 5
7 5 3
2 4 6 8";

        assert_eq!(solve_a(&input), 18);
    }

    #[test]
    fn a_2() {
        let input =
r"10
10";

        assert_eq!(solve_a(&input), 0);
    }

    #[test]
    fn a_3() {
        let input =
r"10 10
10 10";

        assert_eq!(solve_a(&input), 0);
    }

    #[test]
    fn a_4() {
        let input =
r"0 0
0 0";

        assert_eq!(solve_a(&input), 0);
    }

    #[test]
    fn a_5() {
        let input =
r"5	1	9	5
7	5 3
2 4	6	8";

        assert_eq!(solve_a(&input), 18);
    }

    #[test]
    fn b_1() {
        let input =
r"5 9 2 8
9 4 7 3
3 8 6 5";

        assert_eq!(solve_b(&input), 9);
    }
}
