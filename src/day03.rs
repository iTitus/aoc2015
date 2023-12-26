use aoc_runner_derive::{aoc, aoc_generator};
use rustc_hash::FxHashSet;

use crate::common::{Direction, Vec2i};

#[aoc_generator(day3)]
pub fn input_generator(input: &str) -> Vec<Direction> {
    input
        .trim()
        .chars()
        .map(TryInto::try_into)
        .collect::<Result<_, _>>()
        .unwrap()
}

fn visited_houses<'a>(directions: impl IntoIterator<Item = &'a Direction>) -> FxHashSet<Vec2i> {
    let mut current = Vec2i::zeros();
    let mut visited = FxHashSet::from_iter([current]);
    for d in directions {
        current = d.offset(&current);
        visited.insert(current);
    }

    visited
}

#[aoc(day3, part1)]
pub fn part1(input: &[Direction]) -> usize {
    visited_houses(input).len()
}

#[aoc(day3, part2)]
pub fn part2(input: &[Direction]) -> usize {
    let mut visited = visited_houses(input.iter().step_by(2));
    visited.extend(visited_houses(input.iter().skip(1).step_by(2)));
    visited.len()
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_part1_1() {
        assert_eq!(part1(&input_generator(">")), 2);
    }

    #[test]
    fn test_part1_2() {
        assert_eq!(part1(&input_generator("^>v<")), 4);
    }

    #[test]
    fn test_part1_3() {
        assert_eq!(part1(&input_generator("^v^v^v^v^v")), 2);
    }

    #[test]
    fn test_part2_1() {
        assert_eq!(part2(&input_generator("^v")), 3);
    }

    #[test]
    fn test_part2_2() {
        assert_eq!(part2(&input_generator("^>v<")), 3);
    }

    #[test]
    fn test_part2_3() {
        assert_eq!(part2(&input_generator("^v^v^v^v^v")), 11);
    }
}
