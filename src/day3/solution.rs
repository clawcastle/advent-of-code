use std::collections::HashSet;

use itertools::Itertools;

pub fn part1() {
    let input = include_str!("input.txt");

    let in_both_compartments: u64 = input
        .lines()
        .map(BackPack::from_str)
        .filter_map(|comp| comp.get_duplicate_item())
        .map(|c| c.priority())
        .sum();

    println!("{:?}", in_both_compartments);
}

pub fn part2() {
    let input = include_str!("input.txt");

    let priority_sum: u64 = input
        .lines()
        .tuples()
        .map(|(b1, b2, b3)| {
            let mut counts: [u8; 256] = [0; 256];

            for b in &[b1, b2, b3] {
                let mut seen: HashSet<char> = HashSet::new();

                for c in b.chars() {
                    if seen.contains(&c) {
                        continue;
                    }

                    seen.insert(c);
                    let idx = c as usize;
                    counts[idx] += 1;
                    if counts[idx] >= 3 {
                        return c.priority();
                    }
                }
            }
            0
        })
        .sum();

    println!("{priority_sum}");
}

#[derive(Debug)]
pub struct BackPack<T: BackPackItem> {
    compartment_1: Vec<T>,
    compartment_2: Vec<T>,
}

pub trait BackPackItem {
    fn priority(&self) -> u64;
}

impl BackPackItem for char {
    fn priority(&self) -> u64 {
        match self {
            'A'..='Z' => u64::from(*self) - 38,
            'a'..='z' => u64::from(*self) - 96,
            _ => 0,
        }
    }
}

impl BackPack<char> {
    pub fn from_str(line: &str) -> Self {
        let (compartment_1, compartment_2) = line.split_at(line.len() / 2);

        BackPack {
            compartment_1: compartment_1.chars().collect_vec(),
            compartment_2: compartment_2.chars().collect_vec(),
        }
    }

    pub fn get_duplicate_item(&self) -> Option<char> {
        self.compartment_1
            .iter()
            .copied()
            .find(|item| self.compartment_2.contains(item))
    }
}
