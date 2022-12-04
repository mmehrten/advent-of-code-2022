use crate::helpers::Solution;

pub struct Day3;

impl Day3 {
    fn letter_score(&self, letter: char) -> u32 {
        // Rust ASCII 'a' has value 97, value 'A' has value 65
        if letter.is_ascii_lowercase() {
            return letter as u32 - 96;
        } else {
            return letter as u32 - 65 + 27;
        }
    }
}

impl Solution for Day3 {
    /// Return the total priority of duplicate items in a given list of rustsacks.
    ///
    /// A rustsack contents is a single string of upper and lowercase letters. The sack has
    /// two compartments, given by the left and righthand side of each string.
    ///
    /// Lowercase item types a through z have priorities 1 through 26.
    /// Uppercase item types A through Z have priorities 27 through 52.
    fn part_one(&self, data: Vec<String>) -> String {
        let mut score = 0;
        for line in data {
            let comp1 = &line[0..line.len() / 2];
            let comp2 = &line[line.len() / 2..line.len()];
            for letter in comp1.chars() {
                if comp2.contains(letter) {
                    score += self.letter_score(letter);
                    break;
                }
            }
        }
        score.to_string()
    }

    /// Return the total priority of the badges of each elf-group.
    ///
    /// Elf-groups are groupings of three lines in the input. The badges are
    /// items that are common to each elf-group.
    fn part_two(&self, data: Vec<String>) -> String {
        let mut score = 0;
        for idx in (0..data.len()).step_by(3) {
            let elf1 = &data[idx];
            let elf2 = &data[idx + 1];
            let elf3 = &data[idx + 2];
            for letter in elf1.chars() {
                if elf2.contains(letter) && elf3.contains(letter) {
                    score += self.letter_score(letter);
                    break;
                }
            }
        }
        score.to_string()
    }
}

#[cfg(test)]
mod test_solution {
    use crate::day3::Day3;
    use crate::helpers::Solution;
    use crate::helpers::{get_input, InputType};

    #[test]
    fn example_data() {
        let day = Day3;
        let data = get_input(3, InputType::Example);
        assert_eq!(day.part_one(data.clone()), "157".to_string());
        assert_eq!(day.part_two(data), "70".to_string());
    }
}
