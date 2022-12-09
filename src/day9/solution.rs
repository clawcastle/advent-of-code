use std::{collections::HashSet, str::FromStr};

use itertools::Itertools;

pub fn part1() -> usize {
    let input = include_str!("input.txt");

    let moves = input
        .lines()
        .filter_map(|line| Move::from_str(line).ok())
        .collect_vec();

    let mut rope = Rope::default();

    let mut end_positions: HashSet<Position> = HashSet::new();
    end_positions.insert(Position::default());

    for mv in moves {
        for _ in 1..=mv.steps {
            rope = rope.process_move(mv.direction);

            end_positions.insert(rope.end);
        }
    }

    end_positions.len()
}

#[derive(Clone, Copy, Default)]
struct Rope {
    start: Position,
    end: Position,
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

impl Rope {
    fn process_move(&self, direction: Direction) -> Self {
        let start = self.move_start(direction);
        let end = self.move_end(start);

        Rope { start, end }
    }

    fn move_start(&self, direction: Direction) -> Position {
        match direction {
            Direction::Up => Position {
                x: self.start.x,
                y: self.start.y + 1,
            },
            Direction::Down => Position {
                x: self.start.x,
                y: self.start.y - 1,
            },
            Direction::Left => Position {
                x: self.start.x - 1,
                y: self.start.y,
            },
            Direction::Right => Position {
                x: self.start.x + 1,
                y: self.start.y,
            },
        }
    }

    fn move_end(&self, start: Position) -> Position {
        if self.end.is_adjacent(start) {
            return self.end;
        }
        if self.end.x != start.x && self.end.y == start.y {
            return self.end.move_towards_horizontally(start);
        }
        if self.end.y != start.y && self.end.x == start.x {
            return self.end.move_towards_vertically(start);
        }

        self.end
            .move_towards_horizontally(start)
            .move_towards_vertically(start)
    }
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
