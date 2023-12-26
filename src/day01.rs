use std::str::FromStr;

use aoc_runner_derive::{aoc, aoc_generator};
use rustc_hash::FxHashSet;

use crate::common::{lp1_norm, parse_split, Direction, Vec2i};

#[derive(Debug, Copy, Clone)]
pub enum Turn {
    Left,
    Right,
}

impl TryFrom<char> for Turn {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        Ok(match value {
            'L' => Self::Left,
            'R' => Self::Right,
            _ => {
                return Err(());
            }
        })
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Instruction {
    turn: Turn,
    amount: i64,
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            turn: Turn::try_from(s.chars().next().ok_or(())?)?,
            amount: s[1..].parse().map_err(|_| ())?,
        })
    }
}

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<Instruction> {
    parse_split(input, ',').unwrap()
}

fn simulate(instructions: &[Instruction]) -> Vec2i {
    let mut current = (Vec2i::zeros(), Direction::North);
    for i in instructions {
        let dir = match i.turn {
            Turn::Left => current.1.rotate_ccw(),
            Turn::Right => current.1.rotate_cw(),
        };
        let pos = dir.offset_with_amount(&current.0, i.amount);
        current = (pos, dir);
    }

    current.0
}

fn check_for_double_visit(instructions: &[Instruction]) -> Vec2i {
    let mut visited = FxHashSet::default();
    let mut current = (Vec2i::zeros(), Direction::North);
    for i in instructions {
        let dir = match i.turn {
            Turn::Left => current.1.rotate_ccw(),
            Turn::Right => current.1.rotate_cw(),
        };

        let mut pos = current.0;
        for _ in 0..i.amount {
            pos = dir.offset(&pos);
            if !visited.insert(pos) {
                return pos;
            }
        }

        current = (pos, dir);
    }

    unreachable!();
}

#[aoc(day1, part1)]
pub fn part1(input: &[Instruction]) -> i64 {
    lp1_norm(&simulate(input))
}

#[aoc(day1, part2)]
pub fn part2(input: &[Instruction]) -> i64 {
    lp1_norm(&check_for_double_visit(input))
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_part1_1() {
        assert_eq!(part1(&input_generator("R2, L3")), 5);
    }

    #[test]
    fn test_part1_2() {
        assert_eq!(part1(&input_generator("R2, R2, R2")), 2);
    }

    #[test]
    fn test_part1_3() {
        assert_eq!(part1(&input_generator("R5, L5, R5, R3")), 12);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&input_generator("R8, R4, R4, R8")), 4);
    }
}
