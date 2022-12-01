use crate::helpers::Solution;

pub struct Day1;

impl Day1 {
    /// Return the calorie counts carried by all elves, sorted in ascending order
    fn get_calorie_counts(&self, data: Vec<String>) -> Vec<i32> {
        let mut calorie_counts = Vec::new();
        calorie_counts.push(0);
        let mut idx = 0;
        for calorie in data {
            match calorie.parse::<i32>() {
                Ok(value) => calorie_counts[idx] += value,
                _ => {
                    calorie_counts.push(0);
                    idx += 1;
                }
            }
        }
        calorie_counts.sort();
        calorie_counts
    }
}
impl Solution for Day1 {
    /// Given a newline separated list of calories for each elf, finds the elf carrying the most calories.
    ///
    /// E.g.
    /// ```
    /// 100
    ///
    /// 10
    /// 200
    ///
    /// 50
    /// 5
    /// ```
    ///
    /// Would return 210, because elf 2 is carrying the most calories (210).
    fn part_one(&self, data: Vec<String>) -> String {
        self.get_calorie_counts(data).pop().expect("").to_string()
    }

    /// Given a newline separated list of calories for each elf, finds total calories for the top three elves.
    ///
    /// E.g.
    /// ```
    /// 1
    ///
    /// 100
    ///
    /// 10
    /// 200
    ///
    /// 50
    /// 5
    /// ```
    ///
    /// Would return 365, because elves 2, 3, and 4 are carrying 365 callories collectively.
    fn part_two(&self, data: Vec<String>) -> String {
        let calorie_counts = self.get_calorie_counts(data);
        calorie_counts[calorie_counts.len() - 3..]
            .into_iter()
            .sum::<i32>()
            .to_string()
    }
}

#[cfg(test)]
mod test_solution {
    use crate::day1::Day1;
    use crate::helpers::Solution;
    use crate::helpers::{get_input, InputType};

    #[test]
    fn example_data() {
        let day1 = Day1;
        let data = get_input(1, InputType::Example);
        assert_eq!(day1.part_one(data.clone()), "24000".to_string());
        assert_eq!(day1.part_two(data), "45000".to_string());
    }
}
