use std::{collections::HashSet, str::FromStr};

use itertools::Itertools;

pub fn part1() -> usize {
    solve_for_rope_length_n::<2>()
}

pub fn part2() -> usize {
    solve_for_rope_length_n::<10>()
}

fn solve_for_rope_length_n<const N: usize>() -> usize {
    let input = include_str!("input.txt");

    let moves = input
        .lines()
        .filter_map(|line| Move::from_str(line).ok())
        .collect_vec();

    moves
        .iter()
        .flat_map(|mv| (0..mv.steps).map(|_| mv.direction))
        .fold(
            (
                &mut HashSet::<Position>::new(),
                Rope {
                    knots: vec![Position::default(); N],
                },
            ),
            |(positions, rp), dir| {
                let next_rp = rp.process_move(dir);
                positions.insert(*rp.knots.last().expect("Expected at least one knot."));
                (positions, next_rp)
            },
        )
        .0
        .len()
}

#[derive(Clone, Default)]
struct Rope {
    knots: Vec<Position>,
}

impl Rope {
    fn process_move(&self, direction: Direction) -> Self {
        let mut previous_knot: Option<Position> = None;
        let mut new_knots = Vec::with_capacity(self.knots.len());

        for knot in &self.knots {
            let new_knot = match previous_knot {
                Some(prev) => knot.follow(prev),
                None => knot.move_direction(direction),
            };

            previous_knot = Some(new_knot);
            new_knots.push(new_knot);
        }

        Rope { knots: new_knots }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
struct Position {
    x: i64,
    y: i64,
}

#[derive(Clone, Copy)]
struct Move {
    direction: Direction,
    steps: i64,
}

#[derive(Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Position {
    fn is_adjacent(&self, other: Position) -> bool {
        (self.x == other.x || self.x == other.x + 1 || self.x == other.x - 1)
            && (self.y == other.y || self.y == other.y + 1 || self.y == other.y - 1)
    }

    fn move_towards_horizontally(&self, other: Position) -> Self {
        if self.x < other.x {
            Position {
                x: self.x + 1,
                y: self.y,
            }
        } else {
            Position {
                x: self.x - 1,
                y: self.y,
            }
        }
    }

    fn move_towards_vertically(&self, other: Position) -> Self {
        if self.y < other.y {
            Position {
                x: self.x,
                y: self.y + 1,
            }
        } else {
            Position {
                x: self.x,
                y: self.y - 1,
            }
        }
    }

    fn move_direction(&self, direction: Direction) -> Position {
        match direction {
            Direction::Up => Position {
                x: self.x,
                y: self.y + 1,
            },
            Direction::Down => Position {
                x: self.x,
                y: self.y - 1,
            },
            Direction::Left => Position {
                x: self.x - 1,
                y: self.y,
            },
            Direction::Right => Position {
                x: self.x + 1,
                y: self.y,
            },
        }
    }

    fn follow(&self, start: Position) -> Position {
        if self.is_adjacent(start) {
            return *self;
        }
        if self.x != start.x && self.y == start.y {
            return self.move_towards_horizontally(start);
        }
        if self.y != start.y && self.x == start.x {
            return self.move_towards_vertically(start);
        }

        self.move_towards_horizontally(start)
            .move_towards_vertically(start)
    }
}

impl TryFrom<char> for Direction {
    type Error = char;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'U' => Ok(Direction::Up),
            'D' => Ok(Direction::Down),
            'L' => Ok(Direction::Left),
            'R' => Ok(Direction::Right),
            _ => Err(c),
        }
    }
}

impl FromStr for Move {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(direction_char) = s.chars().next() && let Ok(direction) = Direction::try_from(direction_char) {
            if let Ok(steps) = s[2..].parse::<i64>() {
                return Ok(Move {direction, steps});
            }

            return Err(s.to_string());
        }

        Err(s.to_string())
    }
}
