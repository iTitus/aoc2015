use std::ops::Add;

use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use nalgebra::{Dim, DMatrix, Matrix, Scalar, Storage};
use num::{PrimInt, Unsigned};
use num::traits::{CheckedShr, WrappingAdd, WrappingSub};
use rustc_hash::FxHashMap;

#[aoc_generator(day9)]
pub fn input_generator(input: &str) -> DMatrix<i64> {
    let edges: FxHashMap<(&str, &str), i64> = input
        .lines()
        .map(str::trim)
        .filter(|l| !l.is_empty())
        .map(|s| {
            let (from, _, to, _, d) = s.split_whitespace().collect_tuple().ok_or(())?;
            Ok(((from, to).min((to, from)), d.parse().map_err(|_| ())?))
        })
        .collect::<Result<_, ()>>()
        .unwrap();
    let nodes: Vec<_> = edges
        .keys()
        .copied()
        .flat_map(|(a, b)| [a, b])
        .sorted()
        .dedup()
        .collect();

    // add an additional node that has distance 0 to all nodes so it more closely resembles a TSP input
    let n = nodes.len() + 1;
    DMatrix::from_fn(n, n, |r, c| {
        if r == c || r == 0 || c == 0 {
            0
        } else {
            let edge_indices = (r, c).min((c, r));
            let edge = (nodes[edge_indices.0 - 1], nodes[edge_indices.1 - 1]);
            edges[&edge]
        }
    })
}

fn next_bit_permutation<T: Unsigned + PrimInt + CheckedShr + WrappingAdd + WrappingSub>(
    v: T,
) -> Option<T> {
    // https://graphics.stanford.edu/~seander/bithacks.html#NextBitPermutation
    if v.is_zero() || v == T::max_value() {
        return None;
    }

    // t gets v's least significant 0 bits set to 1
    let one = T::one();
    let t = v | (v - one);
    // Next set to 1 the most significant bit to change,
    // set to 0 the least significant ones, and add the necessary 1 bits.
    let tp1 = t.wrapping_add(&one);
    let w = tp1
        | ((!t & tp1)
            .wrapping_sub(&one)
            .checked_shr((v.trailing_zeros() + 1) as _)
            .unwrap_or_else(T::zero));
    if w > v {
        Some(w)
    } else {
        None
    }
}

/// solves the TSP
fn held_karp<T: Scalar + Ord + Add<Output = T>, C: Dim, R: Dim, S: Storage<T, R, C>>(
    m: &Matrix<T, R, C, S>,
) -> T {
    assert!(m.is_square());
    let n = m.nrows();
    assert!(n > 0 && n <= 128);
    let n = n as u8;

    // use u128 as a bitset
    let mut g: FxHashMap<(u128, u8), T> = FxHashMap::default();
    for k in 1..n {
        g.insert((1 << k, k), m[(0, k as usize)].clone());
    }

    for set_size in 2..n {
        // this is smallest number which has "set_size" bits set to 1 and its LSB set to 0
        let mut current = (u128::MAX >> (u128::BITS as usize - set_size as usize)) << 1;
        loop {
            for k in 1..n {
                if (current & (1 << k)) == 0 {
                    continue;
                }

                let current_without_k = current & !(1 << k);
                let x = (1..n)
                    .filter(|l| *l != k && (current & (1 << l)) != 0)
                    .map(|l| g[&(current_without_k, l)].clone() + m[(l as _, k as _)].clone())
                    .min()
                    .unwrap();
                g.insert((current, k), x);
            }

            if let Some(next) = next_bit_permutation(current >> 1)
                .filter(|next| *next < (1 << (n - 1)))
                .map(|next| next << 1)
            {
                current = next;
            } else {
                break;
            }
        }
    }

    let full_set_without_first = ((1 << (n - 1)) - 1) << 1;
    (1..n)
        .map(|k| g[&(full_set_without_first, k)].clone() + m[(k as _, 0)].clone())
        .min()
        .unwrap()
}

#[aoc(day9, part1)]
pub fn part1(input: &DMatrix<i64>) -> i64 {
    let qr = input.full_piv_lu();
    held_karp(input)
}

#[aoc(day9, part2)]
pub fn part2(input: &DMatrix<i64>) -> i64 {
    let reversed = -input;
    -held_karp(&reversed)
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    const INPUT: &str = r#"London to Dublin = 464
London to Belfast = 518
Dublin to Belfast = 141"#;

    #[test]
    fn test_next_bit_permutation_1() {
        assert_eq!(next_bit_permutation(0b0000_0000u8), None);
    }

    #[test]
    fn test_next_bit_permutation_2() {
        assert_eq!(next_bit_permutation(0b0000_0001u8), Some(0b0000_0010));
    }

    #[test]
    fn test_next_bit_permutation_3() {
        assert_eq!(next_bit_permutation(0b1000_0000u8), None);
    }

    #[test]
    fn test_next_bit_permutation_4() {
        assert_eq!(next_bit_permutation(0b0111_1111u8), Some(0b1011_1111));
    }

    #[test]
    fn test_next_bit_permutation_5() {
        assert_eq!(next_bit_permutation(0b1111_1110u8), None);
    }

    #[test]
    fn test_next_bit_permutation_6() {
        assert_eq!(next_bit_permutation(0b1111_1111u8), None);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&input_generator(INPUT)), 605);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&input_generator(INPUT)), 982);
    }
}
