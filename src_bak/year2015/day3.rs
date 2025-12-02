#![allow(unused)]
use std::{collections::HashMap, hash::Hash};

use crate::helpers;

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}

pub fn part1(input: &str) -> i32 {
    let mut visited: HashMap<Point, i32> = HashMap::new();
    let mut pos = Point { x: 0, y: 0 };
    visited.entry(pos).and_modify(|x| *x += 1).or_insert(1);
    for c in input.chars() {
        match c {
            '<' => pos.x -= 1,
            '>' => pos.x += 1,
            'v' => pos.y -= 1,
            '^' => pos.y += 1,
            _ => println!("{c}"),
        }
        visited.entry(pos).and_modify(|x| *x += 1).or_insert(1);
    }
    visited.len() as i32
}

pub fn part2(input: &str) -> i32 {
    let mut visited: HashMap<Point, i32> = HashMap::new();
    let mut santa = Point { x: 0, y: 0 };
    let mut reind = Point { x: 0, y: 0 };
    let mut is_santa = true;

    visited.entry(santa).and_modify(|x| *x += 1).or_insert(1);

    for c in input.chars() {
        let mut pos: Point;
        if is_santa {
            pos = santa;
        } else {
            pos = reind;
        }
        match c {
            '<' => pos.x -= 1,
            '>' => pos.x += 1,
            'v' => pos.y -= 1,
            '^' => pos.y += 1,
            _ => println!("{c}"),
        }
        visited.entry(pos).and_modify(|x| *x += 1).or_insert(1);
        if is_santa {
            santa = pos;
        } else {
            reind = pos;
        }
        is_santa = !is_santa;
    }
    visited.len() as i32
}

pub fn solve() {
    let input = helpers::read_input(2015, 3);
    println!("2015 Day 3 Part 1: {}", part1(&input));
    println!("2015 Day 3 Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = ">";
        assert_eq!(part1(input), 2);
        let input = "^>v<";
        assert_eq!(part1(input), 4);
        let input = "^v^v^v^v^v";
        assert_eq!(part1(input), 2);
    }

    #[test]
    fn test_part2() {
        let input = "^v";
        assert_eq!(part2(input), 3);
        let input = "^>v<";
        assert_eq!(part2(input), 3);
        let input = "^v^v^v^v^v";
        assert_eq!(part2(input), 11);
    }
}
