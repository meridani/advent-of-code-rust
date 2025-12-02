#![allow(unused)]
use itertools::Itertools;

use crate::helpers;

pub fn is_vowel(c: char) -> bool {
    matches!(c, 'a' | 'e' | 'i' | 'o' | 'u')
}

pub fn part1(input: &str) -> i32 {
    input
        .lines()
        .filter(|line| line.chars().filter(|c| is_vowel(*c)).count() >= 3)
        // .inspect(|l| println!("{}", l))
        .filter(|line| {
            line.to_ascii_lowercase()
                .as_bytes()
                .windows(2)
                // .inspect(|f| println!("{:?}", f))
                .any(|pair| pair[0] == pair[1])
        })
        // .inspect(|l| println!("{}", l))
        .filter(|line| {
            !line.contains("ab")
                && !line.contains("cd")
                && !line.contains("pq")
                && !line.contains("xy")
        })
        .count() as i32
}

pub fn part2(input: &str) -> i32 {
    input
        .lines()
        .map(str::as_bytes)
        .collect::<Vec<&[u8]>>()
        .iter()
        .filter(|line| line.windows(3).any(|triple| triple[0] == triple[2]))
        .filter(|(line)| {
            line.windows(2).enumerate().any(|(i, pair)| {
                line.windows(2)
                    .enumerate()
                    .any(|(j, other_pair)| pair == other_pair && (i as i32 - j as i32).abs() > 1)
            })
        })
        .count() as i32
}

pub fn solve() {
    let input = helpers::read_input(2015, 5);
    println!("2015 Day 5 Part 1: {}", part1(&input));
    println!("2015 Day 5 Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "ugknbfddgicrmopn";
        assert_eq!(part1(input), 1);
        let input = "aaa";
        assert_eq!(part1(input), 1);
        let input = "jchzalrnumimnmhp";
        assert_eq!(part1(input), 0);
        let input = "haegwjzuvuyypxyu";
        assert_eq!(part1(input), 0);
    }

    #[test]
    fn test_part2() {
        let input = "qjhvhtzxzqqjkmpb";
        assert_eq!(part2(input), 1);
        let input = "xxyxx";
        assert_eq!(part2(input), 1);
        let input = "uurcxstgmygtbstg";
        assert_eq!(part2(input), 0);
        let input = "ieodomkazucvgmuy";
        assert_eq!(part2(input), 0);
    }
}
