#![allow(unused)]
use core::prelude;

use day1::solver;

fn main() {
    tracing_subscriber::fmt::init();

    let input = day1::solver::get_input();
    let part1 = day1::solver::part1(input);
    let part2 = day1::solver::part2(input);

    println!("---- Day 1 ----");
    println!("Part 1: {part1}");
    println!("Part 2: {part2}")
}
