use crate::common::parse_lines;
use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use std::str::FromStr;

#[aoc_generator(day14)]
pub fn input_generator(input: &str) -> Vec<Reindeer> {
    parse_lines(input).unwrap()
}

#[derive(Debug, Clone)]
pub struct Reindeer {
    pub speed: u64,
    pub speed_time: u64,
    pub rest_time: u64,
}

impl FromStr for Reindeer {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (speed, _, _, speed_time, _, _, _, _, _, _, rest_time, _) =
            s.split_whitespace().skip(3).collect_tuple().ok_or(())?;
        Ok(Self {
            speed: speed.parse().map_err(|_| ())?,
            speed_time: speed_time.parse().map_err(|_| ())?,
            rest_time: rest_time.parse().map_err(|_| ())?,
        })
    }
}

impl Reindeer {
    fn distance_travelled(&self, time: u64) -> u64 {
        let cycle_time = self.speed_time + self.rest_time;
        let cycles = time / cycle_time;
        let partial_time = time % cycle_time;
        let active_time = cycles * self.speed_time + partial_time.min(self.speed_time);
        active_time * self.speed
    }
}

fn solve_part1(reindeers: &[Reindeer], time: u64) -> u64 {
    reindeers
        .iter()
        .map(|r| r.distance_travelled(time))
        .max()
        .unwrap()
}

fn solve_part2(reindeers: &[Reindeer], time: u64) -> u64 {
    let mut stats = reindeers.iter().map(|_| (0u64, 0u64)).collect_vec();
    for t in 1..=time {
        std::iter::zip(reindeers, &mut stats).for_each(|(r, (distance, _score))| {
            *distance = r.distance_travelled(t);
        });
        stats
            .iter_mut()
            .max_set_by_key(|(distance, _score)| *distance)
            .iter_mut()
            .for_each(|(_distance, score)| {
                *score += 1;
            });
    }
    stats
        .into_iter()
        .map(|(_distance, score)| score)
        .max()
        .unwrap()
}

#[aoc(day14, part1)]
pub fn part1(input: &[Reindeer]) -> u64 {
    solve_part1(input, 2503)
}

#[aoc(day14, part2)]
pub fn part2(input: &[Reindeer]) -> u64 {
    solve_part2(input, 2503)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    const INPUT: &str = r#"Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.
Dancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds."#;

    #[test]
    fn test_part1() {
        assert_eq!(solve_part1(&input_generator(INPUT), 1000), 1120);
    }

    #[test]
    fn test_part2() {
        assert_eq!(solve_part2(&input_generator(INPUT), 1000), 689);
    }
}
