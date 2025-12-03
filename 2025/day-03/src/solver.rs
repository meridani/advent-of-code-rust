#![allow(unused)]
use itertools::Itertools;

pub fn get_input() -> &'static str {
    include_str!("../../inputs/03.in")
}
pub fn part1(input: &str) -> u32 {
    let mut total: u32 = 0;
    for line in input.lines() {
        let jolts = line.as_bytes().iter().map(|x| *x - 48).collect::<Vec<u8>>();
        let mut max = 0;
        for i in 0..jolts.len() - 1 {
            for j in i + 1..jolts.len() {
                let num: u32 = (jolts[i] * 10 + jolts[j]).try_into().unwrap();
                if num > max {
                    max = num;
                }
            }
        }
        total += max;
    }
    total
}

pub fn part2(input: &str) -> u128 {
    let mut total: u128 = input
        .lines()
        .map(|bank| {
            let mut batteries: Vec<char> = vec![];
            let mut biggest_index = 0;
            for i in 0..12 {
                let (index, biggest) = &bank[biggest_index..(bank.len() - 11 + i)]
                    .chars()
                    .enumerate()
                    .max_set_by_key(|(_index, battery)| *battery)
                    .first()
                    .cloned()
                    .unwrap();
                batteries.push(*biggest);
                biggest_index = biggest_index + index + 1;
            }

            batteries
                .iter()
                .collect::<String>()
                .parse::<u128>()
                .unwrap()
        })
        .sum();
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "987654321111111
811111111111119
234234234234278
818181911112111";
        assert_eq!(part1(input), 357);
    }

    #[test]
    fn test_part2() {
        let input = "987654321111111
811111111111119
234234234234278
818181911112111
";
        assert_eq!(part2(input), 3121910778619);
    }
}
