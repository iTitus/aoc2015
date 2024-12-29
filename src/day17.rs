use crate::common::parse_lines;
use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day17)]
pub fn input_generator(input: &str) -> Vec<u64> {
    parse_lines(input).unwrap()
}

fn solve(containers: &[u64], amount: u64, max_container_amount: usize) -> usize {
    if amount == 0 {
        return 1;
    } else if containers.is_empty() || max_container_amount == 0 {
        return 0;
    }

    let mut result = solve(&containers[1..], amount, max_container_amount);
    if amount >= containers[0] {
        result += solve(
            &containers[1..],
            amount - containers[0],
            max_container_amount - 1,
        );
    }
    result
}

fn solve_part1(containers: &[u64], amount: u64) -> usize {
    solve(containers, amount, containers.len())
}

fn solve_part2(containers: &[u64], amount: u64) -> usize {
    for i in 0..=containers.len() {
        let result = solve(containers, amount, i);
        if result > 0 {
            return result;
        }
    }

    0
}

#[aoc(day17, part1)]
pub fn part1(input: &[u64]) -> usize {
    solve_part1(input, 150)
}

#[aoc(day17, part2)]
pub fn part2(input: &[u64]) -> usize {
    solve_part2(input, 150)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    const INPUT: &str = r#"20
15
10
5
5"#;

    #[test]
    fn test_part1() {
        assert_eq!(solve_part1(&input_generator(INPUT), 25), 4);
    }

    #[test]
    fn test_part2() {
        assert_eq!(solve_part2(&input_generator(INPUT), 25), 3);
    }
}
