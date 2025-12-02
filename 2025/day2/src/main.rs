#![allow(unused)]
use core::prelude;

use day2::solver;

fn main() {
    tracing_subscriber::fmt::init();

    let input = day2::solver::get_input();

    println!("---- Day  ----");
    let part1 = day2::solver::part1(input);
    println!("Part 1: {part1}");
    let part2 = day2::solver::part2(input);
    println!("Part 2: {part2}")
}
