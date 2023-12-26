use std::str::FromStr;

use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

use crate::common::parse_lines;

#[derive(Debug)]
pub struct Cuboid {
    l: u64,
    w: u64,
    h: u64,
}

impl FromStr for Cuboid {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (l, w, h) = s.trim().split('x').collect_tuple().ok_or(())?;
        Ok(Self {
            l: l.parse().map_err(|_| ())?,
            w: w.parse().map_err(|_| ())?,
            h: h.parse().map_err(|_| ())?,
        })
    }
}

impl Cuboid {
    fn surface(&self) -> u64 {
        2 * (self.l * self.w + self.w * self.h + self.h * self.l)
    }

    fn volume(&self) -> u64 {
        self.l * self.w * self.h
    }

    fn wrapping_paper(&self) -> u64 {
        self.surface() + (self.l * self.w).min(self.w * self.h).min(self.h * self.l)
    }

    fn ribbon(&self) -> u64 {
        2 * (self.l + self.w).min(self.l + self.h).min(self.w + self.h) + self.volume()
    }
}

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<Cuboid> {
    parse_lines(input).unwrap()
}

#[aoc(day2, part1)]
pub fn part1(input: &[Cuboid]) -> u64 {
    input.iter().map(Cuboid::wrapping_paper).sum()
}

#[aoc(day2, part2)]
pub fn part2(input: &[Cuboid]) -> u64 {
    input.iter().map(Cuboid::ribbon).sum()
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_part1_1() {
        assert_eq!(part1(&input_generator("2x3x4")), 58);
    }

    #[test]
    fn test_part1_2() {
        assert_eq!(part1(&input_generator("1x1x10")), 43);
    }

    #[test]
    fn test_part2_1() {
        assert_eq!(part2(&input_generator("2x3x4")), 34);
    }

    #[test]
    fn test_part2_2() {
        assert_eq!(part2(&input_generator("1x1x10")), 14);
    }
}
