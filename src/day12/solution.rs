use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet},
};

use itertools::Itertools;

pub fn part1() {
    let input = include_str!("input.txt");

    let start_position =
        find_position_of_start_or_end('S', input).expect("No start tile present in map.");
    let end_position =
        find_position_of_start_or_end('E', input).expect("No end tile present in map.");

    let grid = input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, c)| Tile {
                    x,
                    y,
                    elevation: c.as_elevation(),
                    dist: distance(x, y, end_position.x, end_position.y),
                })
                .collect_vec()
        })
        .collect_vec();

    let mut q = BinaryHeap::new();
    let mut visited: HashSet<(usize, usize)> = HashSet::new();

    q.push(Reverse(start_position));

    // let mut iterations = 0;

    while let Some(Reverse(tile)) = q.pop() {
        let valid_neighbors = tile.get_valid_adjacent_tiles(&grid);
        if tile == end_position {
            println!("hey");
            break;
        }

        valid_neighbors
            .iter()
            .filter(|n| !&visited.contains(&(n.x, n.y)))
            .for_each(|neighbor| q.push(Reverse(*neighbor)));

        visited.insert((tile.x, tile.y));
    }

    // println!("{:?}, {:?}", start_position, end_position);
}

fn find_position_of_start_or_end(target: char, input: &str) -> Option<Tile> {
    for (i, line) in input.lines().enumerate() {
        for (j, c) in line.chars().enumerate() {
            if c == target {
                return Some(Tile {
                    x: j,
                    y: i,
                    elevation: c.as_elevation(),
                    dist: 0,
                });
            }
        }
    }

    None
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
struct Tile {
    x: usize,
    y: usize,
    elevation: i64,
    dist: usize,
}

impl Ord for Tile {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.dist.cmp(&other.dist)
    }
}

impl PartialOrd for Tile {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Tile {
    fn get_valid_adjacent_tiles(&self, grid: &[Vec<Tile>]) -> Vec<Tile> {
        let candidates = [
            (self.x.wrapping_sub(1), self.y),
            (self.x.wrapping_add(1), self.y),
            (self.x, self.y.wrapping_sub(1)),
            (self.x, self.y.wrapping_add(1)),
        ];

        candidates
            .iter()
            .filter(|(x, y)| *x < grid[0].len() && *y < grid.len())
            .filter(|(x, y)| grid[*y][*x].elevation.abs_diff(self.elevation) < 2)
            .map(|(x, y)| grid[*y][*x])
            .collect_vec()
    }
}

fn distance(x1: usize, y1: usize, x2: usize, y2: usize) -> usize {
    x1.abs_diff(x2) + y1.abs_diff(y2)
}

trait Elevation {
    fn as_elevation(&self) -> i64;
}

impl Elevation for char {
    fn as_elevation(&self) -> i64 {
        match self {
            'S' => 'a' as i64,
            'E' => 'z' as i64,
            _ => *self as i64,
        }
    }
}
