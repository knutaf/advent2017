extern crate aoclib;
use aoclib::*;

fn solve_a(input : &str) -> u32 {
    input.lines().fold(0u32, |sum, line| {
        let (min, max) = line.split_whitespace().map(|num_str| {
            num_str.parse::<u32>().expect("failed to parse num")
        }).fold((None, None), |(min_opt, max_opt), num| {
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

fn main() {
    let input = read_all_stdin();
    //eprintln!("input: {}", input);

    if aoclib::should_solve_puzzle_a() {
        println!("answer: {}", solve_a(&input));
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
}
