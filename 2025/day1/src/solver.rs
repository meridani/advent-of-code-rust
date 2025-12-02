#![allow(unused)]
use itertools::Itertools;
pub fn get_input() -> &'static str {
    include_str!("../../inputs/1.in")
}
pub fn part1(input: &str) -> i32 {
    let mut safe = 50;
    let mut answ = 0;
    for line in input.lines() {
        let dir = line.chars().take(1).collect::<String>();
        let num = line[1..].parse::<i32>().unwrap();
        if dir == 'L'.to_string() {
            safe = (safe - num) % 100;
        } else {
            safe = (safe + num) % 100;
        }
        if safe == 0 {
            answ += 1;
        }
    }
    answ
}

pub fn part2(input: &str) -> i32 {
    let mut safe = 50;
    let mut answ = 0;
    for line in input.lines() {
        let dir = line.chars().take(1).collect::<String>();
        let num = line[1..].parse::<i32>().unwrap();
        for _ in 0..num {
            if dir == 'L'.to_string() {
                safe = (safe - 1) % 100;
            } else {
                safe = (safe + 1) % 100;
            }
            if safe == 0 {
                answ += 1;
            }
        }
    }
    answ
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";
        assert_eq!(part1(input), 3);
    }

    #[test]
    fn test_part2() {
        let input = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";
        assert_eq!(part2(input), 6);
    }
    #[test]
    fn test_part2_man() {
        let input = "L500";
        assert_eq!(part2(input), 5);
        let input = "L50
R1000";
        assert_eq!(part2(input), 11);
    }
}
