// https://adventofcode.com/2024/day/1

use std::collections::HashMap;

use iter_tools::Itertools;

pub fn part1() -> () {
    let input = parse_input();

    let result = input.total_distance();

    println!("Day 1 part 1 solution: {}", result);
}

pub fn part2() -> () {
    let input = parse_input();

    let result = input.total_similarity_score();

    println!("Day 1 part 1 solution: {}", result);
}


fn parse_input() -> Input {
    let file_content = std::fs::read_to_string("./input/day1.txt").expect("Couldn't find input file.");

    let parsed_lines: Vec<Vec<i32>> = file_content.lines()
        .map(|line| line.split_whitespace().map(|num_str| num_str.parse::<i32>().expect("Failed to parse as number.")).collect())
        .collect();

    let left: Vec<i32> = parsed_lines.iter().map(|row| row[0]).sorted().collect();
    let right: Vec<i32> = parsed_lines.iter().map(|row| row[1]).sorted().collect();

    Input {
        left,
        right
    }
}

struct Input {
    left: Vec<i32>,
    right: Vec<i32>
}

impl Input {
    fn total_distance(&self) -> i32 {
        self.left.iter().zip(&self.right).map(|(a,b)| (a - b).abs()).sum()
    }

    fn total_similarity_score(&self) -> i32 {
        let mut occurences: HashMap<&i32, i32> = HashMap::new();

        self.right.iter().for_each(|n| {
            *occurences.entry(n).or_insert(0) += 1;
        });

        self.left.iter().map(|n| n * occurences.get(n).unwrap_or(&0)).sum()
    }
}