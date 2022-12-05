#![feature(iter_next_chunk)]
#![feature(let_chains)]
mod day5;

fn main() {
    let result = crate::day5::solution::part1();

    println!("{:?}", result);
}
