#![allow(unused)]
use itertools::Itertools;

use crate::helpers;

fn check_valid(id: &str) -> bool {
    if id.starts_with('0') {
        return false;
    }

    let mid = id.len() / 2;
    let (a, b) = id.split_at(mid);
    if a == b {
        return false;
    }

    true
}
fn check_valid2(id: &str) -> bool {
    for i in 1..id.len() / 2 {
        let sub = &id[0..=i];
        dbg!(sub);
        match id.match_indices(sub).try_len() {
            Ok(x) if x > 1 => return false,
            Ok(_) => (),
            Err(_) => (),
        }
    }
    true
}

pub fn part1(input: &str) -> i128 {
    let mut invalids: i128 = 0;
    let ranges: Vec<_> = input
        .split(',')
        .map(|range| range.trim().split('-').collect::<Vec<_>>())
        .collect();
    for pair in ranges {
        let start = pair[0].parse::<i128>().unwrap();
        let end = pair[1].parse::<i128>().unwrap();

        for i in start..=end {
            if !check_valid(&i.to_string()) {
                invalids += i;
            }
        }
    }
    invalids
}

pub fn part2(input: &str) -> i128 {
    let mut invalids: i128 = 0;
    let ranges: Vec<_> = input
        .split(',')
        .map(|range| range.trim().split('-').collect::<Vec<_>>())
        .collect();
    for pair in ranges {
        let start = pair[0].parse::<i128>().unwrap();
        let end = pair[1].parse::<i128>().unwrap();

        for i in start..=end {
            if !check_valid2(&i.to_string()) {
                invalids += i;
            }
        }
    }
    invalids
}

pub fn solve() {
    let input = helpers::read_input(2025, 2);
    println!("2025 Day 2 Part 1: {}", part1(&input));
    println!("2025 Day 2 Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_valid() {
        assert_eq!(false, check_valid(&11.to_string()));
        assert_eq!(false, check_valid(&22.to_string()));
        assert_eq!(false, check_valid(&1010.to_string()));
        assert_eq!(false, check_valid(&1188511885.to_string()));
        assert_eq!(false, check_valid(&222222.to_string()));
        assert_eq!(false, check_valid(&446446.to_string()));
        assert_eq!(false, check_valid(&38593859.to_string()));
    }
    #[test]
    fn test_check_valid2() {
        assert_eq!(false, check_valid2(&11.to_string()));
        assert_eq!(false, check_valid2(&22.to_string()));
        assert_eq!(false, check_valid2(&999.to_string()));
        assert_eq!(false, check_valid2(&2121212121.to_string()));
        assert_eq!(false, check_valid2(&1188511885.to_string()));
        assert_eq!(false, check_valid2(&222222.to_string()));
        assert_eq!(false, check_valid2(&446446.to_string()));
        assert_eq!(false, check_valid2(&824824824.to_string()));
    }

    #[test]
    fn test_part1() {
        let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
        assert_eq!(part1(input), 1227775554);
    }

    #[test]
    fn test_part2() {
        let input = "";
        assert_eq!(part2(input), 0);
    }
}
