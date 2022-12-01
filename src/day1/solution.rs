use itertools::Itertools;

pub fn part1() -> i64 {
    get_sums().max().unwrap_or_default()
}

pub fn part2() -> i64 {
    get_sums()
        .sorted()
        .rev()
        .next_chunk::<3>()
        .unwrap()
        .into_iter()
        .sum()
}

fn get_sums() -> Box<dyn Iterator<Item = i64>> {
    let input = include_str!("input.txt");

    Box::new(input.split("\n\n").map(|elf| {
        let sum = elf.lines().fold(0, |curr_sum, line| {
            if let Ok(n) = line.parse::<i64>() {
                return curr_sum + n;
            }

            curr_sum
        });

        sum
    }))
}
