// Common helper functions for Advent of Code challenges

use std::fs;

/// Read input file for a given year and day
///
/// # Examples
/// ```
/// use advent_of_code_rust::helpers::read_input;
/// let input = read_input(2015, 1);
/// ```
pub fn read_input(year: u16, day: u8) -> String {
    let path = format!("inputs/{}/{}.in", year, day);
    fs::read_to_string(&path).unwrap_or_else(|_| panic!("Failed to read input file: {}", path))
}

#[cfg(test)]
mod tests {
    use super::*;
}
