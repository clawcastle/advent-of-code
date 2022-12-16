#![feature(iter_next_chunk)]
#![feature(let_chains)]
#![feature(array_windows)]

mod day11;
mod day12;
mod day13;
fn main() {
    let result = crate::day11::solution::part2();

    dbg!(result);
}
