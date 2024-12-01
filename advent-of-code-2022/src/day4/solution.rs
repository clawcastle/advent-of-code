use std::str::FromStr;

use itertools::Itertools;

pub fn part1() -> usize {
    let sections = get_sections();

    sections
        .tuples()
        .map(|(section_1, section_2)| {
            section_1.contains(section_2) || section_2.contains(section_1)
        })
        .filter(|contains| *contains)
        .count()
}

pub fn part2() -> usize {
    let sections = get_sections();

    sections
        .tuples()
        .map(|(section_1, section_2)| {
            section_1.contains_subset(section_2) || section_2.contains_subset(section_1)
        })
        .filter(|contains| *contains)
        .count()
}

fn get_sections() -> Box<dyn Iterator<Item = Section>> {
    let input = include_str!("input.txt");

    Box::new(
        input
            .lines()
            .flat_map(|line| line.splitn(2, ','))
            .map(Section::from_str)
            .filter_map(|x| x.ok()),
    )
}

#[derive(Clone, Copy)]
pub struct Section {
    start: usize,
    end: usize,
}

impl Section {
    pub fn contains(&self, other: Section) -> bool {
        self.start <= other.start && self.end >= other.end
    }

    pub fn contains_subset(&self, other: Section) -> bool {
        (self.start >= other.start && self.start <= other.end)
            || (self.end <= other.end && self.end >= other.start)
    }
}

impl FromStr for Section {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() < 3 {
            return Err(());
        }

        if let Some((start_str, end_str)) = s.splitn(2, '-').collect_tuple() {
            if let Ok((start, end)) = match (start_str.parse::<usize>(), end_str.parse::<usize>()) {
                (Ok(s), Ok(e)) => Ok((s, e)),
                _ => Err(()),
            } {
                return Ok(Section { start, end });
            }
        }

        println!("fuck: {s}");

        Err(())
    }
}
