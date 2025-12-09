#![allow(unused)]

use itertools::Itertools;

pub fn get_input() -> &'static str {
    include_str!("../../inputs/09.in")
}

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord, Hash, Clone)]
struct Point {
    x: i64,
    y: i64,
}

impl Point {
    fn area(&self, other: &Point) -> u64 {
        (self.x.abs_diff(other.x) + 1) * (self.y.abs_diff(other.y) + 1)
    }
}

pub fn part1(input: &str) -> u64 {
    input
        .lines()
        .map(|line| {
            let p: Vec<i64> = line.trim().split(',').map(|c| c.parse().unwrap()).collect();
            Point { x: p[0], y: p[1] }
        })
        .combinations(2)
        .map(|a| a[0].area(&a[1]))
        .max()
        .unwrap()
}
pub fn part2(input: &str) -> u64 {
    let points = input
        .lines()
        .map(|line| {
            let p = line
                .trim()
                .split(',')
                .map(|c| c.parse().unwrap())
                .collect_vec();
            Point { x: p[0], y: p[1] }
        })
        .collect_vec();

    let edges = points
        .iter()
        .circular_tuple_windows()
        .collect::<Vec<(&Point, &Point)>>();

    let max_rect = points
        .iter()
        .tuple_combinations()
        .map(|(a, b)| (a, b, a.area(b)))
        .sorted_by_key(|v| v.2)
        .rev()
        .find(|(a, b, area)| {
            edges.iter().all(|(start, end)| {
                let left_of_rect = a.x.max(b.x) <= start.x.min(end.x);
                let right_of_rect = a.x.min(b.x) >= start.x.max(end.x);
                let above = a.y.max(b.y) <= start.y.min(end.y);
                let below = a.y.min(b.y) >= start.y.max(end.y);
                left_of_rect || right_of_rect || above || below
            })
        });
    max_rect.unwrap().2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3";
        assert_eq!(part1(input), 50);
    }

    #[test]
    fn test_part2() {
        let input = "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3";
        assert_eq!(part2(input), 24);
    }
}
