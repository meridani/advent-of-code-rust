#![allow(unused)]
use std::{collections::HashMap, fmt::Display, vec};

use itertools::Itertools;
use tracing_subscriber::field::debug;

pub fn get_input() -> &'static str {
    include_str!("../../inputs/04.in")
}
pub fn part1(input: &str) -> u32 {
    let mut grid = HashMap::new();
    input.lines().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, c)| {
            grid.insert(
                Point {
                    x: x.try_into().unwrap(),
                    y: y.try_into().unwrap(),
                },
                c,
            );
        })
    });
    grid.iter().fold(0, |acc, (p, c)| {
        let mut sum = 0;
        if *c == '@' {
            for n in p.neighbours() {
                let roll = grid.get(&n).unwrap_or(&'.');
                if *roll == '@' {
                    sum += 1;
                }
            }
            if sum < 4 { acc + 1 } else { acc }
        } else {
            acc
        }
    })
}

pub fn part2(input: &str) -> u64 {
    let mut grid = HashMap::new();
    let mut total: u64 = 0;
    input.lines().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, c)| {
            grid.insert(
                Point {
                    x: x.try_into().unwrap(),
                    y: y.try_into().unwrap(),
                },
                c,
            );
        })
    });
    loop {
        let mut count = 0;
        let newgrid: HashMap<Point, char> = grid
            .iter()
            .map(|(p, c)| {
                let mut sum = 0;
                if *c == '@' {
                    for n in p.neighbours() {
                        let roll = grid.get(&n).unwrap_or(&'.');
                        if *roll == '@' {
                            sum += 1;
                        }
                    }
                    if sum < 4 {
                        count += 1;
                        total += 1;
                        (Point { x: p.x, y: p.y }, '.')
                    } else {
                        (Point { x: p.x, y: p.y }, *c)
                    }
                } else {
                    (Point { x: p.x, y: p.y }, *c)
                }
            })
            .collect();
        if count == 0 {
            break;
        }
        grid = newgrid;
    }
    total
}

#[derive(Eq, PartialEq, Hash, Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn neighbours(&self) -> Vec<Point> {
        let mut n: Vec<Point> = vec![];
        for i in -1..=1 {
            for j in -1..=1 {
                if i == 0 && j == 0 {
                    continue;
                }
                n.push(Point {
                    x: self.x + i,
                    y: self.y + j,
                });
            }
        }
        n
    }
}
impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "x:{},y:{}", self.x, self.y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";
        assert_eq!(part1(input), 13);
    }

    #[test]
    fn test_part2() {
        let input = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";
        assert_eq!(part2(input), 43);
    }
}
