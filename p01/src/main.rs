extern crate aoclib;

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

fn solve_b(input : &str) -> u32 {
    let digits : Vec<u8> = input.chars().map(|c| { c.to_digit(10).expect("failed to parse digit") as u8 }).collect();
    let lookahead_offset = digits.len() / 2;

    digits.iter().enumerate().fold(0u32, |running_sum, (i, &val)| {
        running_sum + if val == digits[(i + lookahead_offset) % digits.len()] {
            u32::from(val)
        } else {
            0u32
        }
    })
}

fn main() {
    let input = aoclib::read_all_stdin();

    let answer =
        if aoclib::should_solve_puzzle_a() {
            solve_a(&input)
        } else {
            solve_b(&input)
        };

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

    #[test]
    fn b_1() {
        assert_eq!(solve_b("1212"), 6);
    }

    #[test]
    fn b_2() {
        assert_eq!(solve_b("1221"), 0);
    }

    #[test]
    fn b_3() {
        assert_eq!(solve_b("123425"), 4);
    }

    #[test]
    fn b_4() {
        assert_eq!(solve_b("123123"), 12);
    }

    #[test]
    fn b_5() {
        assert_eq!(solve_b("12131415"), 4);
    }
}
