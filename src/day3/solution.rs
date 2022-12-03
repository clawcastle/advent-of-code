use itertools::Itertools;

pub fn part1() {
    let input = include_str!("input.txt");

    let in_both_compartments = input
        .lines()
        .map(|line| BackPack::from_input_line(line))
        .filter_map(|comp| comp.item_in_both_compartments())
        .map(|c| (c as u8) - 65)
        .collect_vec();

    println!("{:?}", in_both_compartments);
}

#[derive(Debug)]
pub struct BackPack<'a> {
    compartment_1: &'a str,
    compartment_2: &'a str,
}

impl<'a> BackPack<'a> {
    pub fn from_input_line(line: &'a str) -> Self {
        let (compartment_1, compartment_2) = line.split_at(line.len() / 2);

        BackPack {
            compartment_1,
            compartment_2,
        }
    }

    pub fn item_in_both_compartments(&self) -> Option<char> {
        let in_both_index = self.compartment_2.find(|c| self.compartment_1.contains(c));

        if let Some(idx) = in_both_index {
            return Some(self.compartment_2.chars().collect_vec()[idx]);
        }

        None
    }
}
