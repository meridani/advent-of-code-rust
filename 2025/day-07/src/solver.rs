#![allow(unused)]
use std::{
    collections::{HashMap, HashSet},
    str::Chars,
    thread::LocalKey,
};

use itertools::Itertools;

pub fn get_input() -> &'static str {
    include_str!("../../inputs/07.in")
}
pub fn part1(input: &str) -> u32 {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let start = grid[0].iter().find_position(|&x| x == &'S').unwrap();
    let mut tachyon: HashSet<usize> = HashSet::new();
    tachyon.insert(start.0);
    let mut splits = 0;
    grid.iter().skip(1).for_each(|line| {
        line.iter().enumerate().for_each(|(i, c)| {
            if *c == '^' && tachyon.contains(&i) {
                splits += 1;
                tachyon.insert(i - 1);
                tachyon.insert(i + 1);
                tachyon.remove(&i);
            }
        })
    });
    // tachyon.len() as u32
    splits
}

pub fn part2(input: &str) -> u64 {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let start = grid[0].iter().find_position(|&x| x == &'S').unwrap();
    let mut tachyon: HashMap<usize, u64> = HashMap::new();
    tachyon.insert(start.0, 1);
    let mut splits = 0;
    grid.iter().skip(1).for_each(|line| {
        line.iter().enumerate().for_each(|(i, c)| {
            if *c == '^' && tachyon.contains_key(&i) {
                let count = tachyon.get(&i).cloned().unwrap();
                tachyon
                    .entry(i - 1)
                    .and_modify(|val| {
                        *val += count;
                    })
                    .or_insert(count);
                tachyon
                    .entry(i + 1)
                    .and_modify(|val| {
                        *val += count;
                    })
                    .or_insert(count);

                tachyon.remove(&i);
            }
        })
    });
    // tachyon.len() as u32
    tachyon.values().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";
        assert_eq!(part1(input), 21);
    }

    #[test]
    fn test_part2() {
        let input = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";
        assert_eq!(part2(input), 40);
    }
}
