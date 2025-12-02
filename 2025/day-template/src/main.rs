#![allow(unused)]
use core::prelude;

use {{crate_name}}::solver;

fn main() {
    tracing_subscriber::fmt::init();

    let input = {{crate_name}}::solver::get_input();

    println!("---- Day {{day}} ----");
    let part1 = {{crate_name}}::solver::part1(input);
    println!("Part 1: {part1}");
    let part2 = {{crate_name}}::solver::part2(input);
    println!("Part 2: {part2}")
}
