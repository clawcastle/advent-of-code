#![feature(iter_next_chunk)]
#![feature(let_chains)]
#![feature(array_windows)]

mod day6;
mod day9;

fn main() {
    let result = crate::day9::solution::part2();

    println!("{:?}", result);
}
