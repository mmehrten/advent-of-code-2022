#!/bin/bash
set -e

DAY=$1

if [ -z "${DAY}" ]; then
    echo "Day required"
    exit 1
fi

if [ ! -e inputs/Example-${DAY}.txt ]; then
    echo "OK" >> inputs/Example-${DAY}.txt
fi

if [ ! -e inputs/Challenge-${DAY}.txt ]; then
    echo "OK" >> inputs/Challenge-${DAY}.txt
fi

if [ ! -e src/day${DAY}.rs ]; then
{
    cat << EOF
use crate::helpers::Solution;

pub struct Day${DAY};

impl Solution for Day${DAY} {
    
    fn part_one(&self, data: Vec<String>) -> String {
        "0".to_string()
    }

    fn part_two(&self, data: Vec<String>) -> String {
        "0".to_string()
    }
}

#[cfg(test)]
mod test_solution {
    use crate::day${DAY}::Day${DAY};
    use crate::helpers::Solution;
    use crate::helpers::{get_input, InputType};

    #[test]
    fn example_data() {
        let day = Day${DAY};
        let data = get_input(${DAY}, InputType::Example);
        assert_eq!(day.part_one(data.clone()), "0".to_string());
        assert_eq!(day.part_two(data), "0".to_string());
    }
}
EOF
} >> src/day${DAY}.rs
fi