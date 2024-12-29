use crate::common::parse_lines;
use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use rustc_hash::FxHashMap;
use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct Sue {
    number: u64,
    properties: FxHashMap<String, u64>,
}

impl FromStr for Sue {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (name, properties) = s.split_once(':').ok_or(())?;
        let (_, number) = name.split_whitespace().collect_tuple().ok_or(())?;
        Ok(Self {
            number: number.parse().map_err(|_| ())?,
            properties: properties
                .split(',')
                .map(str::trim)
                .filter(|s| !s.is_empty())
                .map(|s| {
                    let (k, v) = s.split_once(':').ok_or(())?;
                    Ok((k.trim().to_string(), v.trim().parse().map_err(|_| ())?))
                })
                .collect::<Result<_, _>>()?,
        })
    }
}

#[aoc_generator(day16)]
pub fn input_generator(input: &str) -> Vec<Sue> {
    parse_lines(input).unwrap()
}

fn solve_part1(sues: &[Sue], known_properties: &FxHashMap<String, u64>) -> u64 {
    sues.iter()
        .filter(|&sue| {
            known_properties.iter().all(|(k, &expected_value)| {
                sue.properties.get(k).is_none_or(|&v| v == expected_value)
            })
        })
        .exactly_one()
        .unwrap()
        .number
}

fn solve_part2(sues: &[Sue], known_properties: &FxHashMap<String, u64>) -> u64 {
    sues.iter()
        .filter(|&sue| {
            known_properties.iter().all(|(k, &expected_value)| {
                sue.properties.get(k).is_none_or(|&v| match k.as_str() {
                    "cats" | "trees" => v > expected_value,
                    "pomeranians" | "goldfish" => v < expected_value,
                    _ => v == expected_value,
                })
            })
        })
        .exactly_one()
        .unwrap()
        .number
}

fn expected_properties() -> FxHashMap<String, u64> {
    FxHashMap::from_iter([
        ("children".into(), 3),
        ("cats".into(), 7),
        ("samoyeds".into(), 2),
        ("pomeranians".into(), 3),
        ("akitas".into(), 0),
        ("vizslas".into(), 0),
        ("goldfish".into(), 5),
        ("trees".into(), 3),
        ("cars".into(), 2),
        ("perfumes".into(), 1),
    ])
}

#[aoc(day16, part1)]
pub fn part1(input: &[Sue]) -> u64 {
    solve_part1(input, &expected_properties())
}

#[aoc(day16, part2)]
pub fn part2(input: &[Sue]) -> u64 {
    solve_part2(input, &expected_properties())
}
