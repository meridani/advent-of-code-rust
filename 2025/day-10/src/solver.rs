#![allow(unused)]
use std::vec;

use itertools::Itertools;

pub fn get_input() -> &'static str {
    include_str!("../../inputs/10.in")
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Machine {
    target: Vec<bool>,
    buttons: Vec<Vec<usize>>,
    joltages: Vec<u32>,
}

impl Machine {
    fn new(line: &str) -> Self {
        let mut target = Vec::new();
        let mut buttons = Vec::new();
        let mut joltages = Vec::new();

        line.split(' ')
            .collect::<Vec<&str>>()
            .iter()
            .for_each(|&x| match x {
                x if x.starts_with("[") => {
                    target = x
                        .trim_matches(['[', ']'])
                        .chars()
                        .map(|l| l != '.')
                        .collect_vec();
                }
                x if x.starts_with("(") => {
                    buttons.push(
                        x.trim_matches(['(', ')'])
                            .split(',')
                            .map(|x| x.parse::<usize>().unwrap())
                            .collect_vec(),
                    );
                }
                x if x.starts_with("{") => {
                    joltages = x
                        .trim_matches(['{', '}'])
                        .split(',')
                        .map(|x| x.parse::<u32>().unwrap())
                        .collect_vec();
                }
                _ => {}
            });
        Self {
            target,
            buttons,
            joltages,
        }
    }

    fn press_button(&mut self, button_index: usize) {
        for &light_index in &self.buttons[button_index] {
            let light = self
                .target
                .get_mut(light_index)
                .expect("light does not exists!!");
            *light = !*light;
        }
    }
    // calculates which button presses are needed to reach the target, while minimizing the presses
    fn buttons_to_target(self) -> u32 {}
}

pub fn part1(input: &str) -> u32 {
    input
        .lines()
        .map(|x| {
            let mut machine = Machine::new(x);

            machine.buttons_to_target()
        })
        .sum()
}

pub fn part2(input: &str) -> u32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";
        assert_eq!(part1(input), 7);
    }

    #[test]
    fn test_part2() {
        let input = "";
        assert_eq!(part2(input), 0);
    }
}
