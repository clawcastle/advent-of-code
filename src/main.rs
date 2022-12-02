#![feature(iter_next_chunk)]
#![feature(let_chains)]

mod day1;
mod day2;

fn main() {
    let x = crate::day2::solution::part2();

    println!("{x}");
}
