use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

use crate::common::parse_lines;

#[aoc_generator(day5)]
pub fn input_generator(input: &str) -> Vec<String> {
    parse_lines(input).unwrap()
}

fn is_nice_p1(s: &str) -> bool {
    // check for 3 vowels
    if s.chars()
        .filter(|c| matches!(c, 'a' | 'e' | 'i' | 'o' | 'u'))
        .count()
        < 3
    {
        return false;
    }

    // naughty substrings
    if s.contains("ab") || s.contains("cd") || s.contains("pq") || s.contains("xy") {
        return false;
    }

    // check for two duplicate letters in a row
    if !s.chars().tuple_windows().any(|(a, b)| a == b) {
        return false;
    }

    true
}

fn is_nice_p2(s: &str) -> bool {
    // check for two duplicate letters with one in between
    if !s.chars().tuple_windows().any(|(a, _, b)| a == b) {
        return false;
    }

    fn check_for_duplicated_pair(s: &str) -> bool {
        if s.len() >= 4 {
            for i in 0..s.len() - 3 {
                if s[i+2..].contains(&s[i..i + 2]) {
                    return true;
                }
            }
        }

        false
    }

    check_for_duplicated_pair(s)
}

#[aoc(day5, part1)]
pub fn part1(input: &[String]) -> usize {
    input.iter().filter(|s| is_nice_p1(s)).count()
}

#[aoc(day5, part2)]
pub fn part2(input: &[String]) -> usize {
    input.iter().filter(|s| is_nice_p2(s)).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_1() {
        assert!(is_nice_p1("ugknbfddgicrmopn"));
    }

    #[test]
    fn test_part1_2() {
        assert!(is_nice_p1("aaa"));
    }

    #[test]
    fn test_part1_3() {
        assert!(!is_nice_p1("jchzalrnumimnmhp"));
    }

    #[test]
    fn test_part1_4() {
        assert!(!is_nice_p1("haegwjzuvuyypxyu"));
    }

    #[test]
    fn test_part1_5() {
        assert!(!is_nice_p1("dvszwmarrgswjxmb"));
    }

    #[test]
    fn test_part2_1() {
        assert!(is_nice_p2("qjhvhtzxzqqjkmpb"));
    }

    #[test]
    fn test_part2_2() {
        assert!(is_nice_p2("xxyxx"));
    }

    #[test]
    fn test_part2_3() {
        assert!(!is_nice_p2("uurcxstgmygtbstg"));
    }

    #[test]
    fn test_part2_4() {
        assert!(!is_nice_p2("ieodomkazucvgmuy"));
    }
}
