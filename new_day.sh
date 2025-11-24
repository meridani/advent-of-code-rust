#!/bin/bash
# Helper script to create a new day solution

if [ $# -ne 2 ]; then
    echo "Usage: ./new_day.sh YEAR DAY"
    echo "Example: ./new_day.sh 2015 2"
    exit 1
fi

YEAR=$1
DAY=$2

# Create directories if they don't exist
mkdir -p "src/year${YEAR}"
mkdir -p "inputs/${YEAR}"

# Create the solution file if it doesn't exist
SOLUTION_FILE="src/year${YEAR}/day${DAY}.rs"
if [ -f "$SOLUTION_FILE" ]; then
    echo "Solution file already exists: $SOLUTION_FILE"
else
    cat > "$SOLUTION_FILE" << EOF
#![allow(unused)]
use crate::helpers;

pub fn part1(input: &str) -> i32 {
    // TODO: Implement part 1
    0
}

pub fn part2(input: &str) -> i32 {
    // TODO: Implement part 2
    0
}

pub fn solve() {
    let input = helpers::read_input(${YEAR}, ${DAY});
    println!("${YEAR} Day ${DAY} Part 1: {}", part1(&input));
    println!("${YEAR} Day ${DAY} Part 2: {}", part2(&input));
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
EOF
    echo "Created solution file: $SOLUTION_FILE"
fi

# Create input file if it doesn't exist
INPUT_FILE="inputs/${YEAR}/${DAY}.in"
if [ -f "$INPUT_FILE" ]; then
    echo "Input file already exists: $INPUT_FILE"
else
    touch "$INPUT_FILE"
    echo "Created input file: $INPUT_FILE (empty - add your puzzle input)"
fi

# Check if module is declared in mod.rs
MOD_FILE="src/year${YEAR}/mod.rs"
if [ ! -f "$MOD_FILE" ]; then
    cat > "$MOD_FILE" << EOF
// Year ${YEAR} solutions

pub mod day${DAY};
EOF
    echo "Created module file: $MOD_FILE"
elif ! grep -q "pub mod day${DAY};" "$MOD_FILE"; then
    echo "pub mod day${DAY};" >> "$MOD_FILE"
    echo "Added day${DAY} to $MOD_FILE"
else
    echo "Module day${DAY} already declared in $MOD_FILE"
fi

# Check if year module is declared in lib.rs
if ! grep -q "pub mod year${YEAR};" "src/lib.rs"; then
    echo "pub mod year${YEAR};" >> "src/lib.rs"
    echo "Added year${YEAR} to src/lib.rs"
else
    echo "Module year${YEAR} already declared in src/lib.rs"
fi

echo ""
echo "Setup complete! Next steps:"
echo "1. Add your puzzle input to: $INPUT_FILE"
echo "2. Implement your solution in: $SOLUTION_FILE"
echo "3. Add to main.rs: year${YEAR}::day${DAY}::solve();"
echo "4. Run with: cargo run"
