use crate::common::parse_lines;
use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use rustc_hash::FxHashMap;
use std::str::FromStr;

#[aoc_generator(day13)]
pub fn input_generator(input: &str) -> Vec<HappinessChange> {
    parse_lines(input).unwrap()
}

#[derive(Debug, Clone)]
pub struct HappinessChange {
    pub person: String,
    pub neighbor: String,
    pub happiness: i64,
}

impl FromStr for HappinessChange {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (person, _, sign, happiness, _, _, _, _, _, _, neighbor) = s
            .trim()
            .trim_end_matches('.')
            .split_whitespace()
            .collect_tuple()
            .ok_or(())?;
        let happiness: i64 = happiness.parse().map_err(|_| ())?;
        Ok(Self {
            person: person.to_string(),
            neighbor: neighbor.to_string(),
            happiness: if sign == "lose" {
                -happiness
            } else {
                happiness
            },
        })
    }
}

fn score<S: AsRef<str>>(mapped: &FxHashMap<(&str, &str), i64>, seating: &[S]) -> i64 {
    seating
        .iter()
        .circular_tuple_windows()
        .map(|(p, n)| {
            mapped
                .get(&(p.as_ref(), n.as_ref()))
                .copied()
                .unwrap_or_default()
                + mapped
                    .get(&(n.as_ref(), p.as_ref()))
                    .copied()
                    .unwrap_or_default()
        })
        .sum()
}

fn solve_part1(input: &[HappinessChange]) -> i64 {
    let mapped: FxHashMap<(&str, &str), i64> = input
        .iter()
        .map(|c| ((c.person.as_str(), c.neighbor.as_str()), c.happiness))
        .collect();
    let people = mapped
        .keys()
        .flat_map(|&(a, b)| [a, b])
        .sorted_unstable()
        .dedup()
        .collect_vec();
    people
        .iter()
        .permutations(people.len())
        .map(|s| score(&mapped, &s))
        .max()
        .unwrap()
}

#[aoc(day13, part1)]
pub fn part1(input: &[HappinessChange]) -> i64 {
    solve_part1(input)
}

#[aoc(day13, part2)]
pub fn part2(input: &[HappinessChange]) -> i64 {
    let mut new_input = input.to_vec();
    new_input.push(HappinessChange {
        person: "<me>".to_string(),
        neighbor: input[0].person.to_string(),
        happiness: 0,
    });

    solve_part1(&new_input)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    const INPUT: &str = r#"Alice would gain 54 happiness units by sitting next to Bob.
Alice would lose 79 happiness units by sitting next to Carol.
Alice would lose 2 happiness units by sitting next to David.
Bob would gain 83 happiness units by sitting next to Alice.
Bob would lose 7 happiness units by sitting next to Carol.
Bob would lose 63 happiness units by sitting next to David.
Carol would lose 62 happiness units by sitting next to Alice.
Carol would gain 60 happiness units by sitting next to Bob.
Carol would gain 55 happiness units by sitting next to David.
David would gain 46 happiness units by sitting next to Alice.
David would lose 7 happiness units by sitting next to Bob.
David would gain 41 happiness units by sitting next to Carol."#;

    #[test]
    fn test_part1() {
        assert_eq!(part1(&input_generator(INPUT)), 330);
    }
}
