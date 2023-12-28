use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

#[aoc_generator(day10)]
pub fn input_generator(input: &str) -> Vec<u8> {
    input
        .trim()
        .chars()
        .map(|c| c.to_digit(10).map(|n| n as u8))
        .collect::<Option<_>>()
        .unwrap()
}

fn look_and_say(it: impl IntoIterator<Item = u8>) -> impl Iterator<Item = u8> {
    it.into_iter()
        .dedup_with_count()
        .flat_map(|(count, element)| [count as u8, element])
}

fn apply_look_and_say(n: usize, input: &[u8]) -> Vec<u8> {
    let mut result: Vec<_> = look_and_say(input.iter().copied()).collect();
    for _ in 1..n {
        result = look_and_say(result.into_iter()).collect();
    }
    result
}

#[aoc(day10, part1)]
pub fn part1(input: &[u8]) -> usize {
    apply_look_and_say(40, input).len()
}

#[aoc(day10, part2)]
pub fn part2(input: &[u8]) -> usize {
    apply_look_and_say(50, input).len()
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_part1_1() {
        assert_eq!(apply_look_and_say(1, &input_generator("1")), vec![1, 1]);
    }
}
