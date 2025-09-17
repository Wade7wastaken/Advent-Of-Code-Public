use std::collections::HashMap;

use lib::{itertools::Itertools, select};

fn main() {
    let input = include_str!("./input.txt").trim();
    println!("{}", part1(input));
    println!("{}", part2(input));
}

#[derive(Debug, Clone, Copy)]
enum Dest {
    Bot(u8),
    Output(u8),
}

impl Default for Dest {
    fn default() -> Self {
        Self::Bot(0)
    }
}

fn parse_dest(dest: &str, num: u8) -> Dest {
    match dest {
        "output" => Dest::Output(num),
        "bot" => Dest::Bot(num),
        _ => panic!(),
    }
}

#[derive(Debug, Clone, Default)]
struct Bot {
    chips: Vec<u8>,
    dest_low: Dest,
    dest_high: Dest,
}

fn parse_bots(input: &str) -> HashMap<u8, Bot> {
    let mut bots: HashMap<u8, Bot> = HashMap::new();
    for l in input.lines() {
        let mut words = l.split_ascii_whitespace();
        if l.starts_with('v') {
            let (value, bot_num) = words
                .filter_map(|w| w.parse().ok())
                .collect_tuple()
                .unwrap();
            bots.entry(bot_num).or_default().chips.push(value);
        } else {
            let (bot_num, low, low_num, high, high_num) = select!(words; 1, 5, 6, 10, 11);
            let bot = bots.entry(bot_num.parse().unwrap()).or_default();

            bot.dest_low = parse_dest(low, low_num.parse().unwrap());
            bot.dest_high = parse_dest(high, high_num.parse().unwrap());
        }
    }
    bots
}

fn part1(input: &str) -> u32 {
    let mut bots = parse_bots(input);

    let target = vec![17, 61];

    loop {
        let iter = bots
            .clone()
            .into_iter()
            .filter(|(_, bot)| bot.chips.len() == 2);

        for (bot_num, mut bot) in iter {
            bot.chips.sort_unstable();
            if bot.chips == target {
                return u32::from(bot_num);
            }
            if let Dest::Bot(bot_dest) = bot.dest_low {
                bots.get_mut(&bot_dest).unwrap().chips.push(bot.chips[0]);
            }
            if let Dest::Bot(bot_dest) = bot.dest_high {
                bots.get_mut(&bot_dest).unwrap().chips.push(bot.chips[1]);
            }
            bots.get_mut(&bot_num).unwrap().chips.clear();
        }
    }
}

fn process_chip(dest: Dest, bots: &mut HashMap<u8, Bot>, outputs: &mut HashMap<u8, u8>, chip: u8) {
    match dest {
        Dest::Bot(bot_dest) => {
            bots.get_mut(&bot_dest).unwrap().chips.push(chip);
        }
        Dest::Output(output_dest) => {
            outputs.insert(output_dest, chip);
        }
    }
}

fn part2(input: &str) -> u32 {
    let mut bots = parse_bots(input);

    let mut outputs = HashMap::new();

    let mut did_anything = true;

    while did_anything {
        let iter = bots
            .clone()
            .into_iter()
            .filter(|(_, bot)| bot.chips.len() == 2);

        did_anything = false;
        for (bot_num, mut bot) in iter {
            did_anything = true;
            bot.chips.sort_unstable();
            process_chip(bot.dest_low, &mut bots, &mut outputs, bot.chips[0]);
            process_chip(bot.dest_high, &mut bots, &mut outputs, bot.chips[1]);
            bots.get_mut(&bot_num).unwrap().chips.clear();
        }
    }

    [0, 1, 2]
        .iter()
        .map(|i| u32::from(outputs.remove(i).unwrap()))
        .product()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day() {
        let input = include_str!("./input.txt").trim();
        assert_eq!(part1(input), 113);
        assert_eq!(part2(input), 12803);
    }
}
