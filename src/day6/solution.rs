use itertools::Itertools;

pub fn part1() -> Option<usize> {
    let input = include_str!("input.txt");

    first_index_after_n_unique_characters::<4>(input)
}

pub fn part2() -> Option<usize> {
    let input = include_str!("input.txt");

    first_index_after_n_unique_characters::<14>(input)
}

fn first_index_after_n_unique_characters<const N: usize>(input: &str) -> Option<usize> {
    for (i, window) in input
        .chars()
        .collect_vec()
        .array_windows::<N>()
        .into_iter()
        .enumerate()
    {
        if window.iter().unique().count() == N {
            println!("{:?}", window);
            return Some(i + N);
        }
    }

    None
}
