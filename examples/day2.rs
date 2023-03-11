use itertools::Itertools;
use std::{error::Error, fs};

#[derive(PartialEq, Eq, Copy, Clone)]
enum Action {
    Rock,
    Paper,
    Scissors,
}

impl Action {
    fn score(&self) -> u32 {
        match self {
            Action::Rock => 1,
            Action::Paper => 2,
            Action::Scissors => 3,
        }
    }

    fn outcome(&self, opponent: Action) -> Outcome {
        if *self == opponent {
            Outcome::Draw
        } else if (matches!(self, Action::Rock) && matches!(opponent, Action::Scissors))
            || (matches!(self, Action::Paper) && matches!(opponent, Action::Rock))
            || (matches!(self, Action::Scissors) && matches!(opponent, Action::Paper))
        {
            Outcome::Win
        } else {
            Outcome::Lose
        }
    }
}

#[derive(PartialEq, Eq, Copy, Clone)]
enum Outcome {
    Win,
    Lose,
    Draw,
}

impl Outcome {
    fn score(&self) -> u32 {
        match self {
            Outcome::Win => 6,
            Outcome::Draw => 3,
            Outcome::Lose => 0,
        }
    }

    fn action_for_outcome(&self, opponent_action: Action) -> Action {
        match self {
            Self::Draw => opponent_action,
            Self::Win => match opponent_action {
                Action::Rock => Action::Paper,
                Action::Paper => Action::Scissors,
                Action::Scissors => Action::Rock,
            },
            Self::Lose => match opponent_action {
                Action::Rock => Action::Scissors,
                Action::Paper => Action::Rock,
                Action::Scissors => Action::Paper,
            },
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut part_one_ans = u32::default();
    let mut part_two_ans = u32::default();
    fs::read_to_string("data/day2.txt")?
        .split("\n")
        .for_each(|line| {
            let chars = line.split_whitespace().collect_vec();
            assert_eq!(chars.len(), 2);
            let opponent_action = match chars[0] {
                "A" => Action::Rock,
                "B" => Action::Paper,
                "C" => Action::Scissors,
                _ => unreachable!(),
            };

            // Part one: 2nd col is action
            let own_action = match chars[1] {
                "X" => Action::Rock,
                "Y" => Action::Paper,
                "Z" => Action::Scissors,
                _ => unreachable!(),
            };
            let mut score = own_action.score();
            score += own_action.outcome(opponent_action).score();
            part_one_ans += score;

            // Part two: 2nd col is outcome
            let outcome = match chars[1] {
                "X" => Outcome::Lose,
                "Y" => Outcome::Draw,
                "Z" => Outcome::Win,
                _ => unreachable!(),
            };
            let mut score = outcome.score();
            score += outcome.action_for_outcome(opponent_action).score();
            part_two_ans += score;
        });
    println!("{}", part_one_ans);
    println!("{}", part_two_ans);
    Ok(())
}
