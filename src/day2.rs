use crate::helpers::Solution;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq)]
enum WinState {
    Win,
    Draw,
    Lose,
}

impl WinState {
    fn get_move<'a>(&'a self, other: &'a RPS) -> &RPS {
        if self == &WinState::Draw {
            return other.clone();
        }
        if self == &WinState::Lose {
            return match other {
                &RPS::Rock => &RPS::Scissors,
                &RPS::Paper => &RPS::Rock,
                &RPS::Scissors => &RPS::Paper,
            };
        }
        return match other {
            &RPS::Rock => &RPS::Paper,
            &RPS::Paper => &RPS::Scissors,
            &RPS::Scissors => &RPS::Rock,
        };
    }
}

impl FromStr for WinState {
    type Err = ();
    fn from_str(input: &str) -> Result<WinState, Self::Err> {
        match input {
            "X" => Ok(WinState::Lose),
            "Y" => Ok(WinState::Draw),
            "Z" => Ok(WinState::Win),
            _ => Err(()),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum RPS {
    Rock,
    Paper,
    Scissors,
}

impl RPS {
    fn win_loss_score(&self, other: &RPS) -> usize {
        if self == other {
            return 3;
        }
        if self == &RPS::Rock && other == &RPS::Scissors {
            return 6;
        }
        if self == &RPS::Paper && other == &RPS::Rock {
            return 6;
        }
        if self == &RPS::Scissors && other == &RPS::Paper {
            return 6;
        }
        0
    }

    fn type_score(&self) -> usize {
        match self {
            &RPS::Scissors => 3,
            &RPS::Paper => 2,
            &RPS::Rock => 1,
        }
    }

    fn total_score(&self, other: &RPS) -> usize {
        self.win_loss_score(&other) + self.type_score()
    }
}

impl FromStr for RPS {
    type Err = ();
    fn from_str(input: &str) -> Result<RPS, Self::Err> {
        match input {
            "A" => Ok(RPS::Rock),
            "X" => Ok(RPS::Rock),
            "B" => Ok(RPS::Paper),
            "Y" => Ok(RPS::Paper),
            "C" => Ok(RPS::Scissors),
            "Z" => Ok(RPS::Scissors),
            _ => Err(()),
        }
    }
}

pub struct Day2;

impl Solution for Day2 {
    /// Calculates the expected score for a Rock Paper Scissors game if played according to the provided input instructions.
    ///
    /// Input:
    ///
    /// * First column is what your opponent is going to play: A for Rock, B for Paper, and C for Scissors.
    /// * Second column is what you should play in response: X for Rock, Y for Paper, and Z for Scissors.
    ///
    /// Scoring:
    ///
    /// * 1 for Rock
    /// * 2 for Paper
    /// * 3 for Scissors
    ///
    /// + plus the score for the outcome of the round
    ///
    /// * 0 if you lost
    /// * 3 round was a draw
    /// * 6 if you won
    ///
    /// For example, suppose you were given the following strategy guide:
    ///
    /// ```
    /// A Y
    /// B X
    /// C Z
    /// ```
    ///
    /// You would get a total score of 15 (8 + 1 + 6).
    fn part_one(&self, data: Vec<String>) -> String {
        let moves = data
            .iter()
            .map(|line| line.split(' '))
            .flatten()
            .map(|rps| RPS::from_str(rps).expect("Failed to parse RPS value from inputs"))
            .collect::<Vec<RPS>>();
        let mut score = 0;

        for idx in (0..moves.len()).step_by(2) {
            let theirs = &moves[idx];
            let ours = &moves[idx + 1];
            score += ours.total_score(&theirs);
        }
        score.to_string()
    }

    /// Calculates the expected score for a Rock Paper Scissors game if played according to the provided input instructions.
    ///
    /// Input:
    ///
    /// * First column is what your opponent is going to play: A for Rock, B for Paper, and C for Scissors.
    /// * Second column is whether you should win or lose: X for lose, Y for draw, and Z for win.
    ///
    /// Scoring:
    ///
    /// * 1 for Rock
    /// * 2 for Paper
    /// * 3 for Scissors
    ///
    /// + plus the score for the outcome of the round
    ///
    /// * 0 if you lost
    /// * 3 round was a draw
    /// * 6 if you won
    ///
    /// For example, suppose you were given the following strategy guide:
    ///
    /// ```
    /// A Y
    /// B X
    /// C Z
    /// ```
    ///
    /// You would get a total score of 12 (4 + 1 + 7).

    fn part_two(&self, data: Vec<String>) -> String {
        let moves = data
            .iter()
            .map(|line| line.split(' '))
            .flatten()
            .collect::<Vec<&str>>();
        let mut score = 0;

        for idx in (0..moves.len()).step_by(2) {
            let their_move =
                RPS::from_str(&moves[idx]).expect("Failed to parse RPS value from inputs");
            let ours_win_state = WinState::from_str(&moves[idx + 1])
                .expect("Failed to parse WinState value from inputs");
            let our_move = ours_win_state.get_move(&their_move);
            score += our_move.total_score(&their_move);
        }
        score.to_string()
    }
}

#[cfg(test)]
mod test_solution {
    use crate::day2::Day2;
    use crate::helpers::Solution;
    use crate::helpers::{get_input, InputType};

    #[test]
    fn example_data() {
        let day = Day2;
        let data = get_input(2, InputType::Example);
        assert_eq!(day.part_one(data.clone()), "15".to_string());
        assert_eq!(day.part_two(data), "12".to_string());
    }
}
