#![allow(unused)]

use crate::helpers;
use itertools::{Combinations, Itertools};

// 2*l*w + 2*w*h + 2*h*l
pub fn part1(input: &str) -> i32 {
    let boxes = input.lines();
    let mut size = 0;
    for b in boxes {
        let n: Vec<i32> = b.split('x').map(|x| x.parse::<i32>().unwrap()).collect();
        let a = n.into_iter().combinations(2);
        let mut min = i32::MAX;
        for sides in a {
            let area = sides[0] * sides[1];
            if area < min {
                min = area;
            }
            size += area * 2;
        }
        size += min;
    }
    size
}

pub fn part2(input: &str) -> i32 {
    input
        .lines()
        .map(|lines| lines.split("x").map(|x| x.parse::<i32>().unwrap()))
        .map(|sides| sides.sorted().collect())
        .fold(0, |acc, sides: Vec<i32>| {
            acc + sides[0] * 2 + sides[1] * 2 + sides[0] * sides[1] * sides[2]
        })
}

pub fn solve() {
    let input = helpers::read_input(2015, 2);
    println!("2015 Day 2 Part 1: {}", part1(&input));
    println!("2015 Day 2 Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "2x3x4";
        assert_eq!(part1(input), 58);
    }

    #[test]
    fn test_part2() {
        let input = "2x3x4";
        assert_eq!(part2(input), 34);
    }
    #[test]
    fn test_part1110() {
        let input = "1x1x10";
        assert_eq!(part2(input), 14);
    }
}
