#![allow(unused)]
use crate::helpers;

pub fn part1(input: &str) -> i32 {
    // TODO: Implement part 1
    0
}

pub fn part2(input: &str) -> i32 {
    // TODO: Implement part 2
    0
}

pub fn solve() {
    let input = helpers::read_input(2015, 11);
    println!("2015 Day 11 Part 1: {}", part1(&input));
    println!("2015 Day 11 Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "";
        assert_eq!(part1(input), 0);
    }

    #[test]
    fn test_part2() {
        let input = "";
        assert_eq!(part2(input), 0);
    }
}
