use crate::common::parse_lines;
use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use num::Unsigned;
use std::iter::FusedIterator;
use std::str::FromStr;

#[aoc_generator(day15)]
pub fn input_generator(input: &str) -> Vec<Ingredient> {
    parse_lines(input).unwrap()
}

#[derive(Debug, Clone)]
pub struct Ingredient {
    pub capacity: i64,
    pub durability: i64,
    pub flavor: i64,
    pub texture: i64,
    pub calories: i64,
}

impl FromStr for Ingredient {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (_, _, capacity, _, durability, _, flavor, _, texture, _, calories) =
            s.split_whitespace().collect_tuple().ok_or(())?;
        Ok(Self {
            capacity: capacity.trim_end_matches(',').parse().unwrap(),
            durability: durability.trim_end_matches(',').parse().unwrap(),
            flavor: flavor.trim_end_matches(',').parse().unwrap(),
            texture: texture.trim_end_matches(',').parse().unwrap(),
            calories: calories.trim_end_matches(',').parse().unwrap(),
        })
    }
}

#[derive(Clone)]
#[must_use = "iterators are lazy and do nothing unless consumed"]
pub struct ConstantSumIter<T> {
    len: usize,
    sum: T,
    last: Option<Result<Vec<T>, ()>>,
}

impl<T: Unsigned + Copy> ConstantSumIter<T> {
    pub fn new(len: usize, sum: T) -> Self {
        let mut next = vec![T::zero(); len];
        if len > 0 {
            next[len - 1] = sum;
        }
        Self {
            len,
            sum,
            last: None,
        }
    }
}

impl<T: Unsigned + Copy> Iterator for ConstantSumIter<T> {
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Self::Item> {
        match &mut self.last {
            Some(Err(_)) => None,
            None => {
                let mut next = vec![T::zero(); self.len];
                if self.len > 0 {
                    next[self.len - 1] = self.sum;
                    self.last = Some(Ok(next.clone()));
                } else {
                    self.last = Some(Err(()));
                }
                Some(next)
            }
            Some(last) => {
                let last = last.as_mut().unwrap();
                if last[0] == self.sum {
                    self.last = Some(Err(()));
                    return None;
                }

                debug_assert!(!self.sum.is_zero() && self.len >= 2);
                let last_index = last.len() - 1;
                let mut remaining_sum = T::zero();
                for i in (1..=last_index).rev() {
                    remaining_sum = remaining_sum + last[i];
                    if !remaining_sum.is_zero() {
                        debug_assert!(last[i] == remaining_sum);
                        last[i - 1] = last[i - 1] + T::one();
                        for x in &mut last[i..last_index] {
                            *x = T::zero();
                        }
                        // TODO: maybe cache i-1 as the next starting index for the loop if this will set the last element to zero
                        last[last_index] = remaining_sum - T::one();
                        break;
                    }
                }

                Some(last.clone())
            }
        }
    }
}

impl<T: Unsigned + Copy> FusedIterator for ConstantSumIter<T> {}

fn constant_sum_sequences<T: Unsigned + Copy>(len: usize, sum: T) -> ConstantSumIter<T> {
    ConstantSumIter::new(len, sum)
}

#[aoc(day15, part1)]
pub fn part1(ingredients: &[Ingredient]) -> i64 {
    constant_sum_sequences(ingredients.len(), 100u8)
        .map(|amounts| {
            let (c, d, f, t) = ingredients.iter().zip(&amounts).fold(
                (0i64, 0i64, 0i64, 0i64),
                |(c, d, f, t), (i, &n)| {
                    let n = n as i64;
                    (
                        c + n * i.capacity,
                        d + n * i.durability,
                        f + n * i.flavor,
                        t + n * i.texture,
                    )
                },
            );
            c.max(0) * d.max(0) * f.max(0) * t.max(0)
        })
        .max()
        .unwrap()
}

#[aoc(day15, part2)]
pub fn part2(ingredients: &[Ingredient]) -> i64 {
    constant_sum_sequences(ingredients.len(), 100u8)
        .filter(|amounts| {
            ingredients
                .iter()
                .zip(amounts)
                .fold(0i64, |cal, (i, &n)| cal + n as i64 * i.calories)
                == 500
        })
        .map(|amounts| {
            let (c, d, f, t) = ingredients.iter().zip(&amounts).fold(
                (0i64, 0i64, 0i64, 0i64),
                |(c, d, f, t), (i, &n)| {
                    let n = n as i64;
                    (
                        c + n * i.capacity,
                        d + n * i.durability,
                        f + n * i.flavor,
                        t + n * i.texture,
                    )
                },
            );
            c.max(0) * d.max(0) * f.max(0) * t.max(0)
        })
        .max()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    const INPUT: &str = r#"Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8
Cinnamon: capacity 2, durability 3, flavor -2, texture -1, calories 3"#;

    #[test]
    fn test_sum_iter_0() {
        let mut it = constant_sum_sequences(0, 3u64);
        assert_eq!(it.next(), Some(vec![]));
        assert_eq!(it.next(), None);
    }

    #[test]
    fn test_sum_iter_1() {
        let mut it = constant_sum_sequences(1, 3u64);
        assert_eq!(it.next(), Some(vec![3]));
        assert_eq!(it.next(), None);
    }

    #[test]
    fn test_sum_iter_2_0() {
        let mut it = constant_sum_sequences(2, 0u64);
        assert_eq!(it.next(), Some(vec![0, 0]));
        assert_eq!(it.next(), None);
    }

    #[test]
    fn test_sum_iter_2_1() {
        let mut it = constant_sum_sequences(2, 1u64);
        assert_eq!(it.next(), Some(vec![0, 1]));
        assert_eq!(it.next(), Some(vec![1, 0]));
        assert_eq!(it.next(), None);
    }

    #[test]
    fn test_sum_iter_2_2() {
        let mut it = constant_sum_sequences(2, 2u64);
        assert_eq!(it.next(), Some(vec![0, 2]));
        assert_eq!(it.next(), Some(vec![1, 1]));
        assert_eq!(it.next(), Some(vec![2, 0]));
        assert_eq!(it.next(), None);
    }

    #[test]
    fn test_sum_iter_3_0() {
        let mut it = constant_sum_sequences(3, 0u64);
        assert_eq!(it.next(), Some(vec![0, 0, 0]));
        assert_eq!(it.next(), None);
    }

    #[test]
    fn test_sum_iter_3_1() {
        let mut it = constant_sum_sequences(3, 1u64);
        assert_eq!(it.next(), Some(vec![0, 0, 1]));
        assert_eq!(it.next(), Some(vec![0, 1, 0]));
        assert_eq!(it.next(), Some(vec![1, 0, 0]));
        assert_eq!(it.next(), None);
    }

    #[test]
    fn test_sum_iter_3_2() {
        let mut it = constant_sum_sequences(3, 2u64);
        assert_eq!(it.next(), Some(vec![0, 0, 2]));
        assert_eq!(it.next(), Some(vec![0, 1, 1]));
        assert_eq!(it.next(), Some(vec![0, 2, 0]));
        assert_eq!(it.next(), Some(vec![1, 0, 1]));
        assert_eq!(it.next(), Some(vec![1, 1, 0]));
        assert_eq!(it.next(), Some(vec![2, 0, 0]));
        assert_eq!(it.next(), None);
    }

    #[test]
    fn test_sum_iter_3_3() {
        let mut it = constant_sum_sequences(3, 3u64);
        assert_eq!(it.next(), Some(vec![0, 0, 3]));
        assert_eq!(it.next(), Some(vec![0, 1, 2]));
        assert_eq!(it.next(), Some(vec![0, 2, 1]));
        assert_eq!(it.next(), Some(vec![0, 3, 0]));
        assert_eq!(it.next(), Some(vec![1, 0, 2]));
        assert_eq!(it.next(), Some(vec![1, 1, 1]));
        assert_eq!(it.next(), Some(vec![1, 2, 0]));
        assert_eq!(it.next(), Some(vec![2, 0, 1]));
        assert_eq!(it.next(), Some(vec![2, 1, 0]));
        assert_eq!(it.next(), Some(vec![3, 0, 0]));
        assert_eq!(it.next(), None);
    }

    #[test]
    fn test_sum_iter_3_4() {
        let mut it = constant_sum_sequences(3, 4u64);
        assert_eq!(it.next(), Some(vec![0, 0, 4]));
        assert_eq!(it.next(), Some(vec![0, 1, 3]));
        assert_eq!(it.next(), Some(vec![0, 2, 2]));
        assert_eq!(it.next(), Some(vec![0, 3, 1]));
        assert_eq!(it.next(), Some(vec![0, 4, 0]));
        assert_eq!(it.next(), Some(vec![1, 0, 3]));
        assert_eq!(it.next(), Some(vec![1, 1, 2]));
        assert_eq!(it.next(), Some(vec![1, 2, 1]));
        assert_eq!(it.next(), Some(vec![1, 3, 0]));
        assert_eq!(it.next(), Some(vec![2, 0, 2]));
        assert_eq!(it.next(), Some(vec![2, 1, 1]));
        assert_eq!(it.next(), Some(vec![2, 2, 0]));
        assert_eq!(it.next(), Some(vec![3, 0, 1]));
        assert_eq!(it.next(), Some(vec![3, 1, 0]));
        assert_eq!(it.next(), Some(vec![4, 0, 0]));
        assert_eq!(it.next(), None);
    }

    #[test]
    fn test_sum_iter_4_0() {
        let mut it = constant_sum_sequences(4, 0u64);
        assert_eq!(it.next(), Some(vec![0, 0, 0, 0]));
        assert_eq!(it.next(), None);
    }

    #[test]
    fn test_sum_iter_4_1() {
        let mut it = constant_sum_sequences(4, 1u64);
        assert_eq!(it.next(), Some(vec![0, 0, 0, 1]));
        assert_eq!(it.next(), Some(vec![0, 0, 1, 0]));
        assert_eq!(it.next(), Some(vec![0, 1, 0, 0]));
        assert_eq!(it.next(), Some(vec![1, 0, 0, 0]));
        assert_eq!(it.next(), None);
    }

    #[test]
    fn test_sum_iter_4_2() {
        let mut it = constant_sum_sequences(4, 2u64);
        assert_eq!(it.next(), Some(vec![0, 0, 0, 2]));
        assert_eq!(it.next(), Some(vec![0, 0, 1, 1]));
        assert_eq!(it.next(), Some(vec![0, 0, 2, 0]));
        assert_eq!(it.next(), Some(vec![0, 1, 0, 1]));
        assert_eq!(it.next(), Some(vec![0, 1, 1, 0]));
        assert_eq!(it.next(), Some(vec![0, 2, 0, 0]));
        assert_eq!(it.next(), Some(vec![1, 0, 0, 1]));
        assert_eq!(it.next(), Some(vec![1, 0, 1, 0]));
        assert_eq!(it.next(), Some(vec![1, 1, 0, 0]));
        assert_eq!(it.next(), Some(vec![2, 0, 0, 0]));
        assert_eq!(it.next(), None);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&input_generator(INPUT)), 62842880);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&input_generator(INPUT)), 57600000);
    }
}
