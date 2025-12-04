#![allow(unused)]
use core::prelude;

use day_04::solver;

fn main() {
    tracing_subscriber::fmt::init();

    let input = day_04::solver::get_input();

    println!("---- Day 04 ----");
    let part1 = day_04::solver::part1(input);
    println!("Part 1: {part1}");
    let part2 = day_04::solver::part2(input);
    println!("Part 2: {part2}")
}
