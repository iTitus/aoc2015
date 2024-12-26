use aoc_runner_derive::aoc;
use itertools::Itertools;

fn next(s: impl Into<String>) -> String {
    let mut s = s.into();
    assert!(s.is_ascii());
    let bs = unsafe { s.as_bytes_mut() };
    for b in bs.iter_mut().rev() {
        loop {
            *b += 1;
            if *b != b'i' && *b != b'o' && *b != b'l' {
                break;
            }
        }

        if *b <= b'z' {
            break;
        } else {
            *b = b'a';
        }
    }
    s
}

fn valid(s: impl AsRef<str>) -> bool {
    let s = s.as_ref();
    assert!(s.is_ascii());
    s.as_bytes()
        .iter()
        .all(|&c| c != b'i' && c != b'o' && c != b'l')
        && s.as_bytes()
            .iter()
            .tuple_windows()
            .any(|(&a, &b, &c)| a + 1 == b && b + 1 == c)
        && s.as_bytes()
            .iter()
            .tuple_windows()
            .filter(|(&a, &b)| a == b)
            .map(|(&a, _)| a)
            .unique()
            .count()
            >= 2
}

fn password_iter(s: impl Into<String>) -> impl Iterator<Item = String> {
    let mut current = s.into();
    std::iter::from_fn(move || {
        let mut next = next(&current);
        std::mem::swap(&mut current, &mut next);
        Some(next)
    })
    .filter(|s| valid(s))
}

#[aoc(day11, part1)]
pub fn part1(input: &str) -> String {
    password_iter(input).next().unwrap()
}

#[aoc(day11, part2)]
pub fn part2(input: &str) -> String {
    password_iter(input).nth(1).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_part1() {
        assert_eq!(part1("abcdefgh"), "abcdffaa");
        assert_eq!(part1("ghijklmn"), "ghjaabcc");
    }
}
