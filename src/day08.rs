use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

use crate::common::parse_lines;

#[aoc_generator(day8)]
pub fn input_generator(input: &str) -> Vec<String> {
    parse_lines(input).unwrap()
}

fn unescaped_len(s: &str) -> usize {
    let l = s.len();
    if l < 2 || &s[0..1] != "\"" || &s[l - 1..l] != "\"" {
        panic!("string must be double-quoted");
    }

    let mut chars = s[1..l - 1].chars();
    let mut n = 0;
    loop {
        match chars.next() {
            None => break,
            Some('\\') => match chars.next() {
                Some('\\') | Some('"') => n += 1,
                Some('x') => match chars.next_tuple() {
                    Some((a, b)) if a.is_ascii_hexdigit() && b.is_ascii_hexdigit() => n += 1,
                    _ => panic!("invalid hex escape sequence"),
                },
                _ => panic!("unfinished or unknown escape sequence"),
            },
            _ => n += 1,
        }
    }

    n
}

fn escaped_len(s: &str) -> usize {
    let mut chars = s.chars();
    let mut n = 2;
    loop {
        match chars.next() {
            None => break,
            Some('\\') | Some('"') => n += 2,
            _ => n += 1,
        }
    }

    n
}

#[aoc(day8, part1)]
pub fn part1(input: &[String]) -> usize {
    input
        .iter()
        .map(|s| s.len() - unescaped_len(s.as_str()))
        .sum()
}

#[aoc(day8, part2)]
pub fn part2(input: &[String]) -> usize {
    input
        .iter()
        .map(|s| escaped_len(s.as_str()) - s.len())
        .sum()
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_part1_1() {
        assert_eq!(unescaped_len(r#""""#), 0);
    }

    #[test]
    fn test_part1_2() {
        assert_eq!(unescaped_len(r#""abc""#), 3);
    }

    #[test]
    fn test_part1_3() {
        assert_eq!(unescaped_len(r#""aaa\"aaa""#), 7);
    }

    #[test]
    fn test_part1_4() {
        assert_eq!(unescaped_len(r#""\x27""#), 1);
    }

    #[test]
    fn test_part2_1() {
        assert_eq!(escaped_len(r#""""#), 6);
    }

    #[test]
    fn test_part2_2() {
        assert_eq!(escaped_len(r#""abc""#), 9);
    }

    #[test]
    fn test_part2_3() {
        assert_eq!(escaped_len(r#""aaa\"aaa""#), 16);
    }

    #[test]
    fn test_part2_4() {
        assert_eq!(escaped_len(r#""\x27""#), 11);
    }
}
