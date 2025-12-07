#![allow(unused)]
use itertools::Itertools;

pub fn get_input() -> &'static str {
    include_str!("../../inputs/06.in")
}
pub fn part1(input: &str) -> u64 {
    let problem: Vec<&str> = input.lines().collect();
    let problem_len = problem.len();
    let operations: Vec<&str> = problem[problem_len - 1].trim().split_whitespace().collect();
    let nums: Vec<Vec<u64>> = problem[0..problem_len - 1]
        .iter()
        .map(|line| {
            line.trim()
                .split_whitespace()
                .map(|x| x.parse().unwrap())
                .collect()
        })
        .collect();
    operations
        .into_iter()
        .enumerate()
        .fold(0, |acc, (i, op)| match op {
            "+" => acc + nums.iter().map(|x| x[i as usize]).sum::<u64>(),
            "*" => acc + nums.iter().map(|x| x[i as usize]).product::<u64>(),
            _ => acc,
        })
}

pub fn part2(input: &str) -> u64 {
    let problem: Vec<&str> = input.lines().collect();
    let problem_len = problem.len();
    let operations: Vec<&str> = problem[problem_len - 1].trim().split_whitespace().collect();
    let nums = problem[0..problem_len - 1].iter();
    for n in nums {
        println!("{n:?}");
    }
    3263827
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "123 328  51 64
 45 64  387 23
  6 98  215 314
*   +   *   + ";
        assert_eq!(part1(input), 4277556);
    }

    #[test]
    fn test_part2() {
        let input = "123 328  51 64 \n 45 64  387 23 \n  6 98  215 314\n*   +   *   +  ";
        assert_eq!(part2(input), 3263827);
    }
}
