use aoc_runner_derive::aoc;

#[aoc(day1, part1)]
pub fn part1(input: &[u8]) -> i64 {
    input.iter().fold(0i64, |a, &e| match e {
        b'(' => a + 1,
        b')' => a - 1,
        _ => unreachable!(),
    })
}

#[aoc(day1, part2)]
pub fn part2(input: &[u8]) -> usize {
    input
        .iter()
        .enumerate()
        .try_fold(0i64, |a, (i, &e)| {
            Ok(match e {
                b'(' => a + 1,
                b')' => {
                    let a = a - 1;
                    if a < 0 {
                        return Err(i + 1);
                    }
                    a
                }
                _ => unreachable!(),
            })
        })
        .unwrap_err()
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_part1_1() {
        assert_eq!(part1(b"(())"), 0);
    }

    #[test]
    fn test_part1_2() {
        assert_eq!(part1(b"()()"), 0);
    }

    #[test]
    fn test_part1_3() {
        assert_eq!(part1(b"((("), 3);
    }

    #[test]
    fn test_part1_4() {
        assert_eq!(part1(b"(()(()("), 3);
    }

    #[test]
    fn test_part1_5() {
        assert_eq!(part1(b"))((((("), 3);
    }

    #[test]
    fn test_part1_6() {
        assert_eq!(part1(b"())"), -1);
    }

    #[test]
    fn test_part1_7() {
        assert_eq!(part1(b"))("), -1);
    }

    #[test]
    fn test_part1_8() {
        assert_eq!(part1(b")))"), -3);
    }

    #[test]
    fn test_part1_9() {
        assert_eq!(part1(b")())())"), -3);
    }

    #[test]
    fn test_part2_1() {
        assert_eq!(part2(b")"), 1);
    }

    #[test]
    fn test_part2_2() {
        assert_eq!(part2(b"()())"), 5);
    }
}
