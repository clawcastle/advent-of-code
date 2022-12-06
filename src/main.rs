#![feature(iter_next_chunk)]
#![feature(let_chains)]
#![feature(array_windows)]

mod day6;

fn main() {
    let result = crate::day6::solution::part2();

    println!("{:?}", result);
}
