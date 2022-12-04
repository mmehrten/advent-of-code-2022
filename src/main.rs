mod day1;
mod day2;
mod day3;
mod helpers;
use crate::helpers::{get_input, InputType, Solution};

/// Solution for Advent of Code day N
/// # Arguments
///
/// * `day` - The AoC day to run
///
/// # Returns
///
/// Nothing, prints the solution answers.
fn main() {
    let day = std::env::args()
        .nth(1)
        .expect("Expected day to run")
        .parse::<usize>()
        .expect("Expected input to be an integer day");

    let example = get_input(day, InputType::Example);
    let challenge = get_input(day, InputType::Challenge);

    let cls: Option<Box<dyn Solution>> = match day {
        1 => Some(Box::new(day1::Day1)),
        2 => Some(Box::new(day2::Day2)),
        3 => Some(Box::new(day3::Day3)),
        _ => panic!("Day {} not implemented!", day),
    };
    let cls = cls.expect("Error processing days!");
    println!("Part 1 Example: {:}", cls.part_one(example.clone()));
    println!("Part 1 Challenge: {:}", cls.part_one(challenge.clone()));
    println!("Part 2 Example: {:}", cls.part_two(example));
    println!("Part 2 Challenge: {:}", cls.part_two(challenge));
}
