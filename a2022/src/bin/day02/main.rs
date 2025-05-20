use lib::{itertools::Itertools, tern};

fn main() {
    let input = include_str!("./input.txt").trim();
    println!("{}", part1(input));
    println!("{}", part2(input));
}

#[derive(Debug, PartialEq)]
enum Choice {
    Rock,
    Paper,
    Scissors,
}

enum Outcome {
    Win,
    Loss,
    Tie,
}

impl Choice {
    fn from_char(s: &str) -> Choice {
        match s {
            "A" => Self::Rock,
            "B" => Self::Paper,
            "C" => Self::Scissors,
            "X" => Self::Rock,
            "Y" => Self::Paper,
            "Z" => Self::Scissors,
            _ => unreachable!("invalid rps {s}"),
        }
    }
    fn winner(&self, other: &Choice) -> Outcome {
        if self == other {
            return Outcome::Tie;
        }
        let won = *self == Self::Rock && *other == Self::Scissors
            || *self == Self::Paper && *other == Self::Rock
            || *self == Self::Scissors && *other == Self::Paper;
        tern!(won, Outcome::Win, Outcome::Loss)
    }

    // what opponent has already played
    fn response_to_get(self, outcome: &Outcome) -> Choice {
        match outcome {
            Outcome::Tie => self,
            Outcome::Win => match self {
                Self::Rock => Self::Paper,
                Self::Paper => Self::Scissors,
                Self::Scissors => Self::Rock,
            },
            Outcome::Loss => match self {
                Self::Rock => Self::Scissors,
                Self::Paper => Self::Rock,
                Self::Scissors => Self::Paper,
            },
        }
    }

    fn score(&self) -> u32 {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }
}

impl Outcome {
    fn from_char(s: &str) -> Self {
        match s {
            "X" => Self::Loss,
            "Y" => Self::Tie,
            "Z" => Self::Win,
            _ => unreachable!("invalid rps {s}"),
        }
    }

    fn score(&self) -> u32 {
        match self {
            Self::Win => 6,
            Self::Tie => 3,
            Self::Loss => 0,
        }
    }
}

fn part1(input: &str) -> u32 {
    input
        .lines()
        .map(|game| {
            let (opponent, you) = game
                .split_whitespace()
                .map(Choice::from_char)
                .collect_tuple()
                .unwrap();
            you.winner(&opponent).score() + you.score()
        })
        .sum()
}

fn part2(input: &str) -> u32 {
    input
        .lines()
        .map(|game| {
            let (opponent_s, outcome_s) = game.split_once(' ').unwrap();
            let opponent = Choice::from_char(opponent_s);
            let outcome = Outcome::from_char(outcome_s);
            outcome.score() + opponent.response_to_get(&outcome).score()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day() {
        let input = include_str!("./input.txt").trim();
        assert_eq!(part1(input), 13484);
        assert_eq!(part2(input), 13433);
    }
}
