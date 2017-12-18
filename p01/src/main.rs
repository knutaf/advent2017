use std::io::prelude::*;

fn get_input_from_stdin() -> String {
    let mut contents = String::new();
    std::io::stdin().read_to_string(&mut contents).expect("failed to read input from stdin");
    contents
}

fn solve_a(input : &str) -> u32 {
    let (sum, _) = input.chars().fold((0, input.chars().last().expect("empty string")), |(running_sum, last_char), c| {
        //println!("sum: {}, last: {:?}", running_sum, last_char);

        let next_running_sum = running_sum + if last_char == c {
            c.to_digit(10).expect("failed to parse digit")
        } else {
            0
        };

        (next_running_sum, c)
    });

    sum
}

fn main() {
    let input = get_input_from_stdin();
    let answer = solve_a(input.trim());
    println!("answer: {}", answer);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn a_1() {
        assert_eq!(solve_a("1122"), 3);
    }

    #[test]
    fn a_2() {
        assert_eq!(solve_a("1111"), 4);
    }

    #[test]
    fn a_3() {
        assert_eq!(solve_a("1234"), 0);
    }

    #[test]
    fn a_4() {
        assert_eq!(solve_a("91212129"), 9);
    }
}
