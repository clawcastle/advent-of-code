#![feature(iter_next_chunk)]
#![feature(let_chains)]
mod day1;
mod day2;
mod day3;
mod day4;

fn main() {
    let result = crate::day4::solution::part2();

    println!("{result}");
}
