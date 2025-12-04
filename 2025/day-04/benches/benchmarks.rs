use day_04::*;

fn main() {
    divan::main();
}

#[divan::bench]
fn part1() {
    let input = solver::get_input();
    solver::part1(divan::black_box(input));
}

#[divan::bench]
fn part2() {
    let input = solver::get_input();
    solver::part2(divan::black_box(input));
}
