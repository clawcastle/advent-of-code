#![feature(iter_next_chunk)]

mod day1;

fn main() {
    // Day 1, part 1
    let day1_part1_answer = crate::day1::solution::part1();

    println!("{day1_part1_answer}");

    // Day 1, part 2
    let day1_part2_answer = crate::day1::solution::part2();

    println!("{day1_part2_answer}");
}
