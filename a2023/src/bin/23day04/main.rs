use std::collections::{HashMap, HashSet};

use lib::tern;

fn main() {
    let input = include_str!("./input.txt").trim();
    println!("{}", part1(input));
    println!("{}", part2(input));
}

struct Card {
    number: u32,
    your_numbers: HashSet<u32>,
    winning_numbers: HashSet<u32>,
}

fn parse_number_list(nums: &str) -> HashSet<u32> {
    nums.split_whitespace()
        .map(|n| n.parse().unwrap())
        .collect()
}

fn parse_card(s: &str) -> Card {
    let (card_id, numbers) = s.split_once(':').unwrap();
    let number = card_id.split_once(' ').unwrap().1.trim().parse().unwrap();
    let (winning_numbers_str, your_numbers_str) = numbers.split_once('|').unwrap();
    let winning_numbers = parse_number_list(winning_numbers_str);
    let your_numbers = parse_number_list(your_numbers_str);
    Card {
        number,
        your_numbers,
        winning_numbers,
    }
}

fn num_winners(card: &Card) -> u32 {
    card.your_numbers
        .intersection(&card.winning_numbers)
        .count() as u32
}

fn part1(input: &str) -> u32 {
    input
        .lines()
        .map(parse_card)
        .map(|card| num_winners(&card))
        .map(|num_winners| tern!(num_winners == 0, 0, 2u32.pow(num_winners - 1)))
        .sum()
}

fn part2(input: &str) -> u32 {
    let mut scorecards = HashMap::new();

    for card in input.lines().map(parse_card) {
        let num_winners = num_winners(&card);
        let cards_to_add = *scorecards.entry(card.number).or_insert(1);
        for i in (card.number + 1)..=(card.number + num_winners) {
            *scorecards.entry(i).or_insert(1) += cards_to_add;
        }
    }

    scorecards.into_values().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day() {
        let input = include_str!("./input.txt").trim();
        assert_eq!(part1(input), 18653);
        assert_eq!(part2(input), 5921508);
    }
}
