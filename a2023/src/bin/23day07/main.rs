use lib::{itertools::Itertools, tern};

fn main() {
    let input = include_str!("./input.txt").trim();
    println!("{}", part1(input));
    println!("{}", part2(input));
}

#[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Card {
    Ace,
    King,
    Queen,
    Jack,
    Ten,
    Nine,
    Eight,
    Seven,
    Six,
    Five,
    Four,
    Three,
    Two,
    Joker,
}

fn parse_card(card: char) -> Card {
    match card {
        'A' => Card::Ace,
        'K' => Card::King,
        'Q' => Card::Queen,
        'J' => Card::Jack,
        'T' => Card::Ten,
        '9' => Card::Nine,
        '8' => Card::Eight,
        '7' => Card::Seven,
        '6' => Card::Six,
        '5' => Card::Five,
        '4' => Card::Four,
        '3' => Card::Three,
        '2' => Card::Two,
        _ => unreachable!("Unknown card: {}", card),
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Priority {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    High,
}

fn classify_hand(cards: &Vec<Card>) -> Priority {
    let freq = cards.iter().counts();
    let num_jokers = freq.get(&Card::Joker).copied().unwrap_or(0);
    let freq_sorted = freq.into_values().sorted_unstable().collect::<Vec<_>>();

    match (&freq_sorted[..], num_jokers) {
        ([5], _) => Priority::FiveOfAKind,

        ([1, 4], 1 | 4) => Priority::FiveOfAKind,
        ([1, 4], 0) => Priority::FourOfAKind,

        ([2, 3], 2 | 3) => Priority::FiveOfAKind,
        ([2, 3], 0) => Priority::FullHouse,

        ([1, 1, 3], 1 | 3) => Priority::FourOfAKind,
        ([1, 1, 3], 0) => Priority::ThreeOfAKind,

        ([1, 2, 2], 2) => Priority::FourOfAKind,
        ([1, 2, 2], 1) => Priority::FullHouse,
        ([1, 2, 2], 0) => Priority::TwoPair,

        ([1, 1, 1, 2], 1 | 2) => Priority::ThreeOfAKind,
        ([1, 1, 1, 2], 0) => Priority::OnePair,

        ([1, 1, 1, 1, 1], 1) => Priority::OnePair,
        ([1, 1, 1, 1, 1], 0) => Priority::High,

        _ => unreachable!(
            "Invalid hand type; cards: {:?}, freq: {:?}, jokers: {}",
            cards, freq_sorted, num_jokers
        ),
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Hand {
    cards: Vec<Card>,
    priority: Priority,
    bid: u32,
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other
            .priority
            .cmp(&self.priority)
            .then(other.cards[0].cmp(&self.cards[0]))
            .then(other.cards[1].cmp(&self.cards[1]))
            .then(other.cards[2].cmp(&self.cards[2]))
            .then(other.cards[3].cmp(&self.cards[3]))
            .then(other.cards[4].cmp(&self.cards[4]))
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn parse_hand(input: &str, jokers: bool) -> Hand {
    let (cards_str, bid_str) = input.split_whitespace().collect_tuple().unwrap();
    let cards = cards_str
        .chars()
        .map(parse_card)
        .map(|c| tern!(jokers && c == Card::Jack, Card::Joker, c))
        .collect();

    Hand {
        priority: classify_hand(&cards),
        bid: bid_str.parse().unwrap(),
        cards,
    }
}

fn total_wins(input: &str, jokers: bool) -> u32 {
    input
        .lines()
        .map(|input| parse_hand(input, jokers))
        .sorted()
        .enumerate()
        .map(|(rank, hand)| (rank + 1) as u32 * hand.bid)
        .sum()
}

fn part1(input: &str) -> u32 {
    total_wins(input, false)
}

fn part2(input: &str) -> u32 {
    total_wins(input, true)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day() {
        let input = include_str!("./input.txt").trim();
        assert_eq!(part1(input), 250474325);
        assert_eq!(part2(input), 248909434);
    }
}
