#![allow(unused)]
use rangemap::{RangeInclusiveSet, RangeMap};
use std::{
    collections::HashSet,
    ops::{Range, RangeInclusive},
    vec,
};

use itertools::{Itertools, Tuples};

pub fn get_input() -> &'static str {
    include_str!("../../inputs/05.in")
}
pub fn part1(input: &str) -> u64 {
    let parts: Vec<&str> = input.trim().split("\n\n").collect();
    let mut ranges: Vec<(u64, u64)> = vec![];
    for line in parts[0].lines() {
        let ends: Vec<u64> = line.split("-").map(|x| x.parse().unwrap()).collect();
        ranges.push((ends[0], ends[1]));
    }
    let ids: Vec<u64> = parts[1].lines().map(|x| x.parse().unwrap()).collect();

    let mut total = 0;
    for id in ids {
        for r in &ranges {
            if r.0 < id && id <= r.1 {
                total += 1;
                break;
            }
        }
    }

    total
}

pub fn part2(input: &str) -> u64 {
    let parts: Vec<&str> = input.trim().split("\n\n").collect();
    let mut ranges: Vec<(u64, u64)> = vec![];
    for line in parts[0].lines() {
        let ends: Vec<u64> = line.split("-").map(|x| x.parse().unwrap()).collect();
        ranges.push((ends[0], ends[1]));
    }
    let mut total = RangeInclusiveSet::new();
    for r in &ranges {
        total.insert(r.0..=r.1);
    }
    total
        .iter()
        // .inspect(|x| println!("{x:?}"))
        .map(|r| r.end() - r.start() + 1)
        .sum::<u64>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "3-5
10-14
16-20
12-18

1
5
8
11
17
32";
        assert_eq!(part1(input), 3);
    }

    #[test]
    fn test_part2() {
        let input = "3-5
10-14
16-20
12-18

1
5
8
11
17
32";
        assert_eq!(part2(input), 14);
    }
}
