use std::str::FromStr;

use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use rustc_hash::FxHashMap;

#[derive(Debug, Clone)]
pub enum Ref {
    Num(u16),
    Wire(String),
}

impl FromStr for Ref {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();
        Ok(if let Ok(n) = s.parse() {
            Self::Num(n)
        } else {
            Self::Wire(s.parse().map_err(|_| ())?)
        })
    }
}

#[derive(Debug, Clone)]
pub enum Gate {
    Ref(Ref),
    Not(Ref),
    And(Ref, Ref),
    Or(Ref, Ref),
    LShift(Ref, Ref),
    RShift(Ref, Ref),
}

impl FromStr for Gate {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();
        Ok(if let Some(rest) = s.strip_prefix("NOT") {
            Self::Not(rest.parse()?)
        } else if s.contains("AND") {
            let (l, _, r) = s.split_whitespace().collect_tuple().ok_or(())?;
            Self::And(l.parse()?, r.parse()?)
        } else if s.contains("OR") {
            let (l, _, r) = s.split_whitespace().collect_tuple().ok_or(())?;
            Self::Or(l.parse()?, r.parse()?)
        } else if s.contains("LSHIFT") {
            let (l, _, r) = s.split_whitespace().collect_tuple().ok_or(())?;
            Self::LShift(l.parse()?, r.parse()?)
        } else if s.contains("RSHIFT") {
            let (l, _, r) = s.split_whitespace().collect_tuple().ok_or(())?;
            Self::RShift(l.parse()?, r.parse()?)
        } else {
            Self::Ref(s.parse()?)
        })
    }
}

#[aoc_generator(day7)]
pub fn input_generator(input: &str) -> FxHashMap<String, Gate> {
    input
        .lines()
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(|s| {
            let (gate, output) = s.rsplit_once("->").ok_or(())?;
            Ok((output.trim().to_string(), gate.parse()?))
        })
        .collect::<Result<_, ()>>()
        .unwrap()
}

fn evaluate(gates: &FxHashMap<String, Gate>, target: &str) -> u16 {
    fn push_q<'a>(q: &mut Vec<&'a str>, cache: &FxHashMap<&str, u16>, inputs: &[&'a Ref]) {
        q.extend(inputs.iter().rev().filter_map(|input| match input {
            Ref::Wire(s) if !cache.contains_key(s.as_str()) => Some(s.as_str()),
            _ => None,
        }))
    }

    let mut cache: FxHashMap<&str, u16> = FxHashMap::default();
    let mut q = vec![target];
    while let Some(name) = q.pop() {
        let g = &gates[name];
        let get = |input: &Ref| match input {
            Ref::Num(n) => Some(*n),
            Ref::Wire(s) => cache.get(s.as_str()).copied(),
        };
        let result = match g {
            Gate::Ref(input) => get(input),
            Gate::Not(input) => get(input).map(|n| !n),
            Gate::And(l, r) => get(l).and_then(|l| get(r).map(|r| l & r)),
            Gate::Or(l, r) => get(l).and_then(|l| get(r).map(|r| l | r)),
            Gate::LShift(l, r) => get(l).and_then(|l| get(r).map(|r| l << r)),
            Gate::RShift(l, r) => get(l).and_then(|l| get(r).map(|r| l >> r)),
        };
        match result {
            None => {
                q.push(name);
                match g {
                    Gate::Ref(input) => push_q(&mut q, &cache, &[input]),
                    Gate::Not(input) => push_q(&mut q, &cache, &[input]),
                    Gate::And(l, r) => push_q(&mut q, &cache, &[l, r]),
                    Gate::Or(l, r) => push_q(&mut q, &cache, &[l, r]),
                    Gate::LShift(l, r) => push_q(&mut q, &cache, &[l, r]),
                    Gate::RShift(l, r) => push_q(&mut q, &cache, &[l, r]),
                };
            }
            Some(n) => {
                cache.insert(name, n);
            }
        }
    }

    cache[target]
}

#[aoc(day7, part1)]
pub fn part1(input: &FxHashMap<String, Gate>) -> u16 {
    evaluate(input, "a")
}

#[aoc(day7, part2)]
pub fn part2(input: &FxHashMap<String, Gate>) -> u16 {
    let a = evaluate(input, "a");

    let mut cloned = input.clone();
    *cloned.get_mut("b").unwrap() = Gate::Ref(Ref::Num(a));
    evaluate(&cloned, "a")
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    const INPUT: &str = r#"123 -> x
456 -> y
x AND y -> d
x OR y -> e
x LSHIFT 2 -> f
y RSHIFT 2 -> g
NOT x -> h
NOT y -> i"#;

    #[test]
    fn test_part1_1() {
        let input = input_generator(INPUT);
        assert_eq!(evaluate(&input, "d"), 72);
    }

    #[test]
    fn test_part1_2() {
        let input = input_generator(INPUT);
        assert_eq!(evaluate(&input, "e"), 507);
    }

    #[test]
    fn test_part1_3() {
        let input = input_generator(INPUT);
        assert_eq!(evaluate(&input, "f"), 492);
    }

    #[test]
    fn test_part1_4() {
        let input = input_generator(INPUT);
        assert_eq!(evaluate(&input, "g"), 114);
    }

    #[test]
    fn test_part1_5() {
        let input = input_generator(INPUT);
        assert_eq!(evaluate(&input, "h"), 65412);
    }

    #[test]
    fn test_part1_6() {
        let input = input_generator(INPUT);
        assert_eq!(evaluate(&input, "i"), 65079);
    }

    #[test]
    fn test_part1_7() {
        let input = input_generator(INPUT);
        assert_eq!(evaluate(&input, "x"), 123);
    }

    #[test]
    fn test_part1_8() {
        let input = input_generator(INPUT);
        assert_eq!(evaluate(&input, "y"), 456);
    }
}
