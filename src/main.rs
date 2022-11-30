mod day1;
mod helpers;
use helpers::{get_input, InputType};

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
    match day {
        1 => {
            println!("Example: {:}", day1::solution(example));
            println!("Challenge: {:}", day1::solution(challenge));
        }
        _ => panic!("Day {} not implemented!", day),
    }
}
