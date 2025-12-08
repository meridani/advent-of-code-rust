#![allow(unused)]
use std::collections::HashSet;

use itertools::Itertools;

pub fn get_input() -> &'static str {
    include_str!("../../inputs/08.in")
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Point {
    x: i64,
    y: i64,
    z: i64,
}

impl Point {
    fn manhattan_distance(&self, other: &Point) -> u64 {
        ((self.x - other.x).abs() + (self.y - other.y).abs() + (self.z - other.z).abs()) as u64
    }
    fn euclidean_distance(&self, other: &Point) -> i64 {
        ((self.x - other.x).pow(2) + (self.y - other.y).pow(2) + (self.z - other.z).pow(2)).isqrt()
    }
}

pub fn part1(input: &str) -> u32 {
    let points: Vec<Point> = input
        .lines()
        .map(|line| {
            let coords: Vec<i64> = line.split(',').map(|s| s.parse().unwrap()).collect();
            Point {
                x: coords[0],
                y: coords[1],
                z: coords[2],
            }
        })
        .collect();
    let mut circuits: Vec<HashSet<Point>> = vec![];

    let sorted_pairs: Vec<_> = points
        .iter()
        .combinations(2)
        .map(|pair| {
            let dist = pair[0].euclidean_distance(pair[1]);
            (pair[0], pair[1], dist)
        })
        .sorted_by(|&x, &y| Ord::cmp(&x.2, &y.2))
        .collect();

    let mut i = 0;
    for (p1, p2, dist) in sorted_pairs {
        if i < 1000 {
            i += 1;
        } else {
            break;
        }
        let p1_circuit_idx = circuits.iter().position(|circuit| circuit.contains(p1));
        let p2_circuit_idx = circuits.iter().position(|circuit| circuit.contains(p2));

        match (p1_circuit_idx, p2_circuit_idx) {
            (Some(idx1), Some(idx2)) if idx1 == idx2 => {
                // both points already in the same circuit
                continue;
            }
            (Some(idx1), None) => {
                // add p2 to p1's circuit
                circuits[idx1].insert(p2.clone());
            }
            (None, Some(idx2)) => {
                // add p1 to p2's circuit
                circuits[idx2].insert(p1.clone());
            }
            (None, None) => {
                // neither point in any circuit, create a new one
                let mut new_circuit = HashSet::new();
                new_circuit.insert(p1.clone());
                new_circuit.insert(p2.clone());
                circuits.push(new_circuit);
            }
            (Some(idx1), Some(idx2)) => {
                // merge two different circuits
                let circuit2 = circuits.remove(idx2.max(idx1));
                let circuit1_idx = idx1.min(idx2);
                circuits[circuit1_idx].extend(circuit2);
            }
        }
    }
    circuits
        .iter()
        .sorted_by_key(|c| c.len())
        .rev()
        .take(3)
        .map(|c| c.len() as u32)
        // .inspect(|x| println!("{x:?}"))
        .product()
}

pub fn part2(input: &str) -> i64 {
    let points: Vec<Point> = input
        .lines()
        .map(|line| {
            let coords: Vec<i64> = line.split(',').map(|s| s.parse().unwrap()).collect();
            Point {
                x: coords[0],
                y: coords[1],
                z: coords[2],
            }
        })
        .collect();
    let mut circuits: Vec<HashSet<Point>> = vec![];
    for p in &points {
        circuits.push({
            let mut hs = HashSet::new();
            hs.insert(p.clone());
            hs
        });
    }

    let sorted_pairs: Vec<_> = points
        .iter()
        .combinations(2)
        .map(|pair| {
            let dist = pair[0].euclidean_distance(pair[1]);
            (pair[0], pair[1], dist)
        })
        .sorted_by(|&x, &y| Ord::cmp(&x.2, &y.2))
        .collect();
    let mut answ = 0;
    for (p1, p2, dist) in sorted_pairs {
        let p1_circuit_idx = circuits.iter().position(|circuit| circuit.contains(p1));
        let p2_circuit_idx = circuits.iter().position(|circuit| circuit.contains(p2));

        match (p1_circuit_idx, p2_circuit_idx) {
            (Some(idx1), Some(idx2)) if idx1 == idx2 => {
                // both points already in the same circuit
                continue;
            }
            (Some(idx1), None) => {
                // add p2 to p1's circuit
                circuits[idx1].insert(p2.clone());
            }
            (None, Some(idx2)) => {
                // add p1 to p2's circuit
                circuits[idx2].insert(p1.clone());
            }
            (None, None) => {
                // neither point in any circuit, create a new one
                let mut new_circuit = HashSet::new();
                new_circuit.insert(p1.clone());
                new_circuit.insert(p2.clone());
                circuits.push(new_circuit);
            }
            (Some(idx1), Some(idx2)) => {
                // merge two different circuits
                let circuit2 = circuits.remove(idx2.max(idx1));
                let circuit1_idx = idx1.min(idx2);
                circuits[circuit1_idx].extend(circuit2);
            }
        }
        if circuits.len() == 1 {
            answ = p1.x * p2.x;
            break;
        }
    }
    answ
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689";
        assert_eq!(part1(input), 40);
    }

    #[test]
    fn test_part2() {
        let input = "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689";
        assert_eq!(part2(input), 25272);
    }
}
