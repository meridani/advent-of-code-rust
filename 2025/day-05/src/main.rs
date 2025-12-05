#![allow(unused)]
use core::prelude;

use day_05::solver;

fn main() {
    tracing_subscriber::fmt::init();

    let input = day_05::solver::get_input();

    println!("---- Day 05 ----");
    let part1 = day_05::solver::part1(input);
    println!("Part 1: {part1}");
    let part2 = day_05::solver::part2(input);
    println!("Part 2: {part2}")
}
