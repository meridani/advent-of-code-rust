#![allow(unused)]
use std::arch::x86_64;

use crate::helpers;

pub fn part1(input: &str) -> i32 {
    input
        .chars()
        .fold(0, |floor, x| if x == '(' { floor + 1 } else { floor - 1 })
}

pub fn part2(input: &str) -> i32 {
    let mut loc = 0;
    let mut floor = 0;
    let mut iter = input.chars();
    loop {
        let c = iter.next();
        loc += 1;
        if c == Some('(') {
            floor += 1;
        } else {
            if floor == 0 {
                break;
            }
            floor -= 1;
        }
    }
    loc
}

pub fn solve() {
    let input = helpers::read_input(2015, 1);
    println!("2015 Day 1 Part 1: {}", part1(&input));
    println!("2015 Day 1 Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "(())";
        assert_eq!(part1(input), 0);
    }

    #[test]
    fn test_part2() {
        let input = ")";
        assert_eq!(part2(input), 1);
    }
}
