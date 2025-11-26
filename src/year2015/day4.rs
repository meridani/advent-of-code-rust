#![allow(unused)]
use crate::helpers;

pub fn part1(input: &str) -> i64 {
    let mut i: i64 = 0;
    loop {
        let data = format!("{}{}", input.to_string().trim(), &i.to_string());
        let h = format!("{:?}", md5::compute(&data));
        if h[0..5] == *"00000" {
            break;
        }
        i += 1;
    }
    i
}

pub fn part2(input: &str) -> i64 {
    let mut i: i64 = 0;
    loop {
        let data = format!("{}{}", input.to_string().trim(), &i.to_string());
        let h = format!("{:?}", md5::compute(&data));
        if h[0..6] == *"000000" {
            break;
        }
        i += 1;
    }
    i
}

pub fn solve() {
    let input = helpers::read_input(2015, 4);
    println!("2015 Day 4 Part 1: {}", part1(&input));
    println!("2015 Day 4 Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "abcdef";
        assert_eq!(part1(input), 609043);
        let input = "pqrstuv";
        assert_eq!(part1(input), 1048970);
    }

    #[test]
    fn test_part2() {
        let input = "";
        assert_eq!(part2(input), 20412333);
    }
}
