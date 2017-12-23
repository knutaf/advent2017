extern crate aoclib;
use aoclib::*;

fn are_words_anagrams(word1 : &str, word2 : &str) -> bool {
    let mut w1 : Vec<char> = word1.chars().collect();
    let mut w2 : Vec<char> = word2.chars().collect();
    w1.sort();
    w2.sort();
    w1 == w2
}

fn has_two_matching_words<F>(passphrase : &str, words_match : F) -> bool
    where F : Fn(&str, &str) -> bool {
    let words : Vec<&str> = passphrase.split_whitespace().collect();
    words.iter().enumerate().fold(false, |sofar, (i, word1)| {
        sofar || words.iter().skip(i + 1).any(|word2| {
            words_match(word1, word2)
        })
    })
}

fn has_two_same_words(passphrase : &str) -> bool {
    has_two_matching_words(passphrase, std::cmp::PartialEq::eq)
}

fn has_anagrams(passphrase : &str) -> bool {
    has_two_matching_words(passphrase, are_words_anagrams)
}

fn solve<F>(input : &str, is_valid_line : F) -> (u32, u32)
    where F : Fn(&str) -> bool {
    input.lines().fold((0, 0), |(total_lines, valid_lines), line| {
        (
            total_lines + 1,
            valid_lines +
                if is_valid_line(line) {
                    1
                } else {
                    0
                }
        )
    })
}

fn solve_a(input : &str) -> (u32, u32) {
    solve(input, |line| {
        !has_two_same_words(line)
    })
}

fn solve_b(input : &str) -> (u32, u32) {
    solve(input, |line| {
        !has_anagrams(line)
    })
}

fn main() {
    let input = read_all_stdin();
    //eprintln!("input: {}", input);

    let (total, valid) =
        if aoclib::should_solve_puzzle_a() {
            solve_a(&input)
        } else {
            solve_b(&input)
        };

    println!("{} out of {} are valid", valid, total);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn passphrase() {
        assert_eq!(has_two_same_words("aa bb cc dd ee"), false);
        assert_eq!(has_two_same_words("aa bb cc dd aa"), true);
        assert_eq!(has_two_same_words("aa bb cc dd aaa"), false);
    }

    #[test]
    fn anagrams() {
        assert_eq!(are_words_anagrams("abcde", "ecdab"), true);
        assert_eq!(are_words_anagrams("abcde", "abcde"), true);
        assert_eq!(are_words_anagrams("abcde", "abcdef"), false);
        assert_eq!(are_words_anagrams("abcde", "edcba"), true);
        assert_eq!(are_words_anagrams("abcdef", "abcde"), false);
    }

    #[test]
    fn a_1() {
        let input =
r"aa bb cc dd ee
aa bb cc dd aa
aa bb cc dd aaa";

        assert_eq!(solve_a(&input), (3, 2));
    }

    #[test]
    fn b_1() {
        let input =
r"abcde fghij
abcde xyz ecdab
a ab abc abd abf abj
iiii oiii ooii oooi oooo
oiii ioii iioi iiio";

        assert_eq!(solve_b(&input), (5, 3));
    }
}
