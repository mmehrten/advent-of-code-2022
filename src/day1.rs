pub fn solution(data: Vec<String>) -> usize {
    1
}

#[cfg(test)]
mod test_solution {
    use crate::day1::solution;
    use crate::helpers::{get_input, InputType};

    #[test]
    fn example_data() {
        assert_eq!(solution(get_input(1, InputType::Example)), 1);
    }
}
