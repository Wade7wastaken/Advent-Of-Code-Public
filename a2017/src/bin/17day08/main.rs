use std::collections::HashMap;

use lib::itertools::Itertools;

fn main() {
    let input = include_str!("./input.txt").trim();
    println!("{}", part1(input));
    println!("{}", part2(input));
}

fn part1(input: &str) -> i32 {
    let mut registers = HashMap::new();
    for l in input.lines() {
        let (target, instr, amount, _, cond_reg, cond, cond_imm) =
            l.split_ascii_whitespace().collect_tuple().unwrap();
        let cond_value = *registers.entry(cond_reg).or_insert(0);
        let constant = cond_imm.parse().unwrap();
        let should_happen = match cond {
            "==" => cond_value == constant,
            "!=" => cond_value != constant,
            ">" => cond_value > constant,
            "<" => cond_value < constant,
            ">=" => cond_value >= constant,
            "<=" => cond_value <= constant,
            _ => panic!(),
        };
        let amount = amount.parse::<i32>().unwrap();
        if should_happen {
            match instr {
                "inc" => *registers.entry(target).or_insert(0) += amount,
                "dec" => *registers.entry(target).or_insert(0) -= amount,
                _ => panic!(),
            }
        }
    }
    *registers.values().max().unwrap()
}

fn part2(input: &str) -> i32 {
    let mut registers = HashMap::new();

    let mut highest_so_far = i32::MIN;

    for l in input.lines() {
        let (target, instr, amount, _, cond_reg, cond, cond_imm) =
            l.split_ascii_whitespace().collect_tuple().unwrap();
        let cond_value = *registers.entry(cond_reg).or_insert(0);
        let constant = cond_imm.parse().unwrap();
        let should_happen = match cond {
            "==" => cond_value == constant,
            "!=" => cond_value != constant,
            ">" => cond_value > constant,
            "<" => cond_value < constant,
            ">=" => cond_value >= constant,
            "<=" => cond_value <= constant,
            _ => panic!(),
        };
        let amount = amount.parse::<i32>().unwrap();
        if should_happen {
            match instr {
                "inc" => *registers.entry(target).or_insert(0) += amount,
                "dec" => *registers.entry(target).or_insert(0) -= amount,
                _ => panic!(),
            }
        }
        let highest = *registers.values().max().unwrap();
        if highest > highest_so_far {
            highest_so_far = highest;
        }
    }

    highest_so_far
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day() {
        let input = include_str!("./input.txt").trim();
        assert_eq!(part1(input), 5215);
        assert_eq!(part2(input), 6419);
    }
}
