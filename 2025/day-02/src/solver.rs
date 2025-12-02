#![allow(unused)]
use itertools::Itertools;
pub fn get_input() -> &'static str {
    include_str!("../../inputs/02.in")
}

fn check_valid(id: &str) -> bool {
    let mid = id.len() / 2;
    if id[..mid] == id[mid..] {
        return false;
    }

    true
}
fn check_valid2(id: &str) -> bool {
    for i in 1..=id.len() / 2 {
        let equal = id
            .as_bytes()
            .chunks(i)
            .map(|buf| unsafe { str::from_utf8_unchecked(buf) })
            .collect::<Vec<&str>>()
            .iter()
            .all_equal();
        if equal {
            return false;
        }
    }
    true
}

pub fn part1(input: &str) -> u64 {
    let mut invalids: u64 = 0;
    let ranges: Vec<_> = input
        .split(',')
        .map(|range| {
            range
                .trim()
                .split('-')
                .map(|x| x.parse::<u64>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect();
    for pair in ranges {
        let start = pair[0];
        let end = pair[1];

        for i in start..=end {
            if !check_valid(&i.to_string()) {
                invalids += i;
            }
        }
    }
    invalids
}

pub fn part2(input: &str) -> u64 {
    let mut invalids: u64 = 0;
    let ranges: Vec<_> = input
        .split(',')
        .map(|range| {
            range
                .trim()
                .split('-')
                .map(|x| x.parse::<u64>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect();
    for pair in ranges {
        let start = pair[0];
        let end = pair[1];

        for i in start..=end {
            if !check_valid2(&i.to_string()) {
                invalids += i;
            }
        }
    }
    invalids
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
