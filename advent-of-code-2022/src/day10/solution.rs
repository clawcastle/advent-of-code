use itertools::FoldWhile::{Continue, Done};
use itertools::Itertools;
use std::str::FromStr;

const MEASURING_POINTS: [usize; 6] = [20, 60, 100, 140, 180, 220];

pub fn part1() -> i64 {
    let input = include_str!("input.txt");

    let instructions = input
        .lines()
        .filter_map(|line| Instruction::from_str(line).ok())
        .collect_vec();

    MEASURING_POINTS.into_iter().fold(0i64, |sum, point| {
        sum + get_x_value_during_nth_cycle(point, &instructions).0
    })
}

fn get_x_value_during_nth_cycle(n: usize, instructions: &[Instruction]) -> (i64, usize) {
    instructions
        .iter()
        .fold_while((1i64, 0usize), |(x, cycles), instruction| {
            let next = match instruction {
                Instruction::Noop => (x, cycles + instruction.cycles()),
                Instruction::Addx(v) => (x + v, cycles + instruction.cycles()),
            };

            if next.1 >= n {
                Done((x * (n as i64), cycles))
            } else {
                Continue(next)
            }
        })
        .into_inner()
}

#[derive(Clone, Copy)]
enum Instruction {
    Noop,
    Addx(i64),
}

impl Instruction {
    fn cycles(&self) -> usize {
        match self {
            Instruction::Noop => 1,
            Instruction::Addx(_) => 2,
        }
    }
}

impl FromStr for Instruction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "noop" {
            return Ok(Instruction::Noop);
        }

        let parts = s.splitn(2, ' ').collect_vec();

        if parts.len() == 2 && parts[0] == "addx" && let Ok(x) = parts[1].parse::<i64>() {
            return Ok(Instruction::Addx(x));
        }

        Err(s.to_string())
    }
}
