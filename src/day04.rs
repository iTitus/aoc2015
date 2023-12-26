use std::io::Write;

use aoc_runner_derive::aoc;
use md5::{Digest, Md5};

fn find_md5_suffix(prefix: &str, hash_prefix_mask: u128, hash_prefix: u128) -> u64 {
    let hasher = Md5::new_with_prefix(prefix);
    (1..)
        .find(|n| {
            let mut hasher = hasher.clone();
            write!(&mut hasher, "{}", n).unwrap();
            let result = u128::from_be_bytes(hasher.finalize().into());
            (result & hash_prefix_mask) == hash_prefix
        })
        .unwrap()
}

#[aoc(day4, part1)]
pub fn part1(input: &str) -> u64 {
    find_md5_suffix(input, 0xfffff << (128 - 4 * 5), 0)
}

#[aoc(day4, part2)]
pub fn part2(input: &str) -> u64 {
    find_md5_suffix(input, 0xffffff << (128 - 4 * 6), 0)
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    #[ignore]
    fn test_part1_1() {
        assert_eq!(part1("abcdef"), 609043);
    }

    #[test]
    #[ignore]
    fn test_part1_2() {
        assert_eq!(part1("pqrstuv"), 1048970);
    }
}
