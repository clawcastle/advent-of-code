#![feature(iter_next_chunk)]
#![feature(let_chains)]
#![feature(array_windows)]

mod day12;
mod day13;
fn main() {
    let result = crate::day13::solution::part2();

    dbg!(result);
}
