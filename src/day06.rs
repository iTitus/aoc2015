use std::iter::Sum;
use std::str::FromStr;

use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

use crate::common::{parse_lines, parse_vec, Vec2i};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Action {
    TurnOn,
    TurnOff,
    Toggle,
}

impl Action {
    fn apply<L: Light>(&self, l: &mut L) {
        match self {
            Action::TurnOn => l.turn_on(),
            Action::TurnOff => l.turn_off(),
            Action::Toggle => l.toggle(),
        }
    }
}

pub struct Instruction {
    action: Action,
    from: Vec2i,
    to: Vec2i,
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (action, s) = {
            Ok(if let Some(rest) = s.strip_prefix("turn on") {
                (Action::TurnOn, rest)
            } else if let Some(rest) = s.strip_prefix("turn off") {
                (Action::TurnOff, rest)
            } else if let Some(rest) = s.strip_prefix("toggle") {
                (Action::Toggle, rest)
            } else {
                return Err(());
            })
        }?;

        let (from, _, to) = s.split_whitespace().collect_tuple().ok_or(())?;
        let from: Vec2i = parse_vec(from).map_err(|_| ())?;
        let to: Vec2i = parse_vec(to).map_err(|_| ())?;

        let (min_x, max_x) = if from.x <= to.x {
            (from.x, to.x)
        } else {
            (to.x, from.x)
        };
        let (min_y, max_y) = if from.y <= to.y {
            (from.y, to.y)
        } else {
            (to.y, from.y)
        };

        Ok(Self {
            action,
            from: Vec2i::new(min_x, min_y),
            to: Vec2i::new(max_x, max_y),
        })
    }
}

trait Light {
    type Value;

    fn off() -> Self;

    fn turn_on(&mut self);

    fn turn_off(&mut self);

    fn toggle(&mut self);

    fn value(&self) -> <Self as Light>::Value;
}

impl Light for bool {
    type Value = usize;

    fn off() -> Self {
        false
    }

    fn turn_on(&mut self) {
        *self = true;
    }

    fn turn_off(&mut self) {
        *self = false;
    }

    fn toggle(&mut self) {
        *self = !*self;
    }

    fn value(&self) -> <Self as Light>::Value {
        *self as _
    }
}

impl Light for u64 {
    type Value = u64;

    fn off() -> Self {
        0
    }

    fn turn_on(&mut self) {
        *self += 1;
    }

    fn turn_off(&mut self) {
        *self = self.saturating_sub(1)
    }

    fn toggle(&mut self) {
        *self += 2;
    }

    fn value(&self) -> <Self as Light>::Value {
        *self
    }
}

fn simulate<T: Light + Copy, R, const SX: usize, const SY: usize>(instructions: &[Instruction]) -> R
where
    R: Sum<<T as Light>::Value>,
{
    // this is a vec because of stack frame overflow
    let mut grid = vec![T::off(); SX * SY];
    for i in instructions {
        for y in i.from.y..=i.to.y {
            for x in i.from.x..=i.to.x {
                let l = &mut grid[(x as usize) + SX * (y as usize)];
                i.action.apply(l);
            }
        }
    }

    grid.iter().map(Light::value).sum()
}

#[aoc_generator(day6)]
pub fn input_generator(input: &str) -> Vec<Instruction> {
    parse_lines(input).unwrap()
}

#[aoc(day6, part1)]
pub fn part1(input: &[Instruction]) -> usize {
    simulate::<bool, _, 1000, 1000>(input)
}

#[aoc(day6, part2)]
pub fn part2(input: &[Instruction]) -> u64 {
    simulate::<u64, _, 1000, 1000>(input)
}
