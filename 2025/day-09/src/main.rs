#![allow(unused)]
use core::prelude;

use day_09::solver;

fn main() {
    tracing_subscriber::fmt::init();

    let input = day_09::solver::get_input();

    println!("---- Day 09 ----");
    let part1 = day_09::solver::part1(input);
    println!("Part 1: {part1}");
    let part2 = day_09::solver::part2(input);
    println!("Part 2: {part2}")
}
