#![allow(unused)]
use core::prelude;

use day_07::solver;

fn main() {
    tracing_subscriber::fmt::init();

    let input = day_07::solver::get_input();

    println!("---- Day 07 ----");
    let part1 = day_07::solver::part1(input);
    println!("Part 1: {part1}");
    let part2 = day_07::solver::part2(input);
    println!("Part 2: {part2}")
}
