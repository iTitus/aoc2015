use aoc_runner_derive::{aoc, aoc_generator};
use serde_json::Value;

#[aoc_generator(day12)]
pub fn input_generator(input: &str) -> Value {
    serde_json::from_str(input).unwrap()
}

fn for_each_df(v: &Value, mut f: impl FnMut(&Value) -> bool) {
    let mut q = vec![v];
    while let Some(v) = q.pop() {
        if !f(v) {
            continue;
        }
        match v {
            Value::Array(a) => {
                q.extend(a.iter().rev());
            }
            Value::Object(o) => {
                q.extend(o.values().rev());
            }
            _ => {}
        }
    }
}

#[aoc(day12, part1)]
pub fn part1(input: &Value) -> i64 {
    let mut result = 0;
    for_each_df(input, |v| {
        if let Value::Number(n) = v {
            result += n.as_i64().unwrap();
        }
        true
    });
    result
}

#[aoc(day12, part2)]
pub fn part2(input: &Value) -> i64 {
    let mut result = 0;
    for_each_df(input, |v| {
        match v {
            Value::Object(o) => {
                if o.values().any(|v| {
                    if let Value::String(s) = v {
                        if s == "red" {
                            return true;
                        }
                    }
                    false
                }) {
                    return false;
                }
            }
            Value::Number(n) => {
                result += n.as_i64().unwrap();
            }
            _ => {}
        }
        true
    });
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_part1() {
        assert_eq!(part1(&input_generator("[1,2,3]")), 6);
        assert_eq!(part1(&input_generator("{\"a\":2,\"b\":4}")), 6);
        assert_eq!(part1(&input_generator("[[[3]]]")), 3);
        assert_eq!(part1(&input_generator("{\"a\":{\"b\":4},\"c\":-1}")), 3);
        assert_eq!(part1(&input_generator("{\"a\":[-1,1]}")), 0);
        assert_eq!(part1(&input_generator("[-1,{\"a\":1}]")), 0);
        assert_eq!(part1(&input_generator("[]")), 0);
        assert_eq!(part1(&input_generator("{}")), 0);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&input_generator("[1,2,3]")), 6);
        assert_eq!(part2(&input_generator("[1,{\"c\":\"red\",\"b\":2},3]")), 4);
        assert_eq!(
            part2(&input_generator("{\"d\":\"red\",\"e\":[1,2,3,4],\"f\":5}")),
            0
        );
        assert_eq!(part2(&input_generator("[1,\"red\",5]")), 6);
    }
}
