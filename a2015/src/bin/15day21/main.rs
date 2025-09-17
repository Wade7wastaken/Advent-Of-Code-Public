use std::ops::Add;

use lib::{StringTools, itertools::Itertools, num::Integer};

fn main() {
    let input = include_str!("./input.txt").trim();
    println!("{}", part1(input));
    println!("{}", part2(input));
}

#[derive(Debug, Clone)]
struct Entity {
    hp: i32,
    damage: i32,
    armor: i32,
}

impl Entity {
    fn new_player(damage: i32, armor: i32) -> Self {
        Self {
            hp: 100,
            damage,
            armor,
        }
    }
    fn wins_against(self, other: &Entity) -> bool {
        let player_damage = 1.max(self.damage - other.armor);

        let enemy_damage = 1.max(other.damage - self.armor);

        let (mut player_hits_taken, rest) = self.hp.div_mod_floor(&enemy_damage);
        if rest == 0 {
            player_hits_taken -= 1;
        }

        let (mut enemy_hits_taken, rest) = other.hp.div_mod_floor(&player_damage);
        if rest == 0 {
            enemy_hits_taken -= 1;
        }

        player_hits_taken >= enemy_hits_taken
    }
}

#[derive(Debug, Clone, Copy)]
struct Item {
    cost: i32,
    damage: i32,
    armor: i32,
}

impl Add<Item> for Item {
    type Output = Item;
    fn add(self, rhs: Item) -> Self::Output {
        Item {
            cost: self.cost + rhs.cost,
            damage: self.damage + rhs.damage,
            armor: self.armor + rhs.armor,
        }
    }
}

// weapons, armor, rings
fn parse_shop(input: &str) -> (Vec<Item>, Vec<Item>, Vec<Item>) {
    input
        .paragraphs()
        .map(|shop_section| {
            shop_section
                .lines()
                .skip(1)
                .map(|line| {
                    let (cost, damage, armor) = line
                        .trim()
                        .split_ascii_whitespace()
                        .tail(3)
                        .map(|n| n.parse().unwrap())
                        .collect_tuple()
                        .unwrap();
                    Item {
                        cost,
                        damage,
                        armor,
                    }
                })
                .collect_vec()
        })
        .collect_tuple()
        .unwrap()
}

fn parse_boss(input: &str) -> Entity {
    let (hp, damage, armor) = input
        .lines()
        .map(|l| l.split_once(": ").unwrap().1.parse().unwrap())
        .collect_tuple()
        .unwrap();
    Entity { hp, damage, armor }
}

fn run_combinations(mut run_sim: impl FnMut(Item)) {
    let (weapons, armor, rings) = parse_shop(include_str!("./shop.txt"));

    for weapon in weapons {
        run_sim(weapon);
        for ring in &rings {
            run_sim(weapon + *ring);
        }
        for (ring_a, ring_b) in rings.iter().tuple_combinations() {
            run_sim(weapon + *ring_a + *ring_b);
        }
        for armor_piece in &armor {
            run_sim(weapon + *armor_piece);
            for ring in &rings {
                run_sim(weapon + *armor_piece + *ring);
            }
            for (ringa, ringb) in rings.iter().tuple_combinations() {
                run_sim(weapon + *armor_piece + *ringa + *ringb);
            }
        }
    }
}

fn part1(input: &str) -> i32 {
    let boss = parse_boss(input);

    let mut min_cost = i32::MAX;

    let run_sim = |gear: Item| {
        let wins = Entity::new_player(gear.damage, gear.armor).wins_against(&boss);
        if wins && gear.cost < min_cost {
            min_cost = gear.cost;
        }
    };

    run_combinations(run_sim);

    min_cost
}

fn part2(input: &str) -> i32 {
    let mut max_cost = i32::MIN;

    let boss = parse_boss(input);

    let run_sim = |gear: Item| {
        let wins = Entity::new_player(gear.damage, gear.armor).wins_against(&boss);
        if !wins && gear.cost > max_cost {
            max_cost = gear.cost;
        }
    };

    run_combinations(run_sim);

    max_cost
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day() {
        let input = include_str!("./input.txt").trim();
        assert_eq!(part1(input), 78);
        assert_eq!(part2(input), 148);
    }
}
