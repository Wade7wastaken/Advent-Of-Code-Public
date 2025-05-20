use std::{collections::HashMap, hash::Hash};

use lib::{itertools::Itertools, tern};

fn main() {
    let input = include_str!("./input.txt").trim();
    println!("{}", part1(input));
    println!("{}", part2(input));
}

// 1355 - too low

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct GameState {
    player_health: i32,
    player_mana: i32,
    player_mana_spent: i32,
    boss_health: i32,
    boss_damage: i32,
    shield_timer: u32,
    poison_timer: u32,
    recharge_timer: u32,
    hard: bool,
}

impl GameState {
    fn spend_mana(&mut self, n: i32) -> Option<()> {
        self.player_mana -= n;
        self.player_mana_spent += n;

        if self.player_mana < 0 {
            return None;
        }
        Some(())
    }

    fn apply_effects(&mut self) {
        if self.shield_timer > 0 {
            self.shield_timer -= 1;
        }
        if self.poison_timer > 0 {
            self.boss_health -= 3;
            self.poison_timer -= 1;
        }
        if self.recharge_timer > 0 {
            self.player_mana += 101;
            self.recharge_timer -= 1;
        }
    }
}

fn magic_missile(mut state: GameState, cache: &mut HashMap<GameState, Option<i32>>) -> Option<i32> {
    state.spend_mana(53);
    if state.player_mana < 0 {
        return None;
    }

    state.boss_health -= 4;
    boss_turn(state, cache)
}

fn drain(mut state: GameState, cache: &mut HashMap<GameState, Option<i32>>) -> Option<i32> {
    state.spend_mana(73)?;

    state.boss_health -= 2;
    state.player_health += 2;
    boss_turn(state, cache)
}

fn shield(mut state: GameState, cache: &mut HashMap<GameState, Option<i32>>) -> Option<i32> {
    if state.shield_timer != 0 {
        return None;
    }

    state.spend_mana(113)?;

    state.shield_timer = 6;
    boss_turn(state, cache)
}

fn poison(mut state: GameState, cache: &mut HashMap<GameState, Option<i32>>) -> Option<i32> {
    if state.poison_timer != 0 {
        return None;
    }

    state.spend_mana(173)?;

    state.poison_timer = 6;
    boss_turn(state, cache)
}

fn recharge(mut state: GameState, cache: &mut HashMap<GameState, Option<i32>>) -> Option<i32> {
    if state.recharge_timer != 0 {
        return None;
    }

    state.spend_mana(229)?;

    state.recharge_timer = 5;
    boss_turn(state, cache)
}

fn cached<T: Eq + Hash + Clone, O: Clone>(
    state: T,
    f: impl FnOnce(T, &mut HashMap<T, O>) -> O,
    cache: &mut HashMap<T, O>,
) -> O {
    if let Some(cached) = cache.get(&state) {
        return cached.clone();
    }
    let output = f(state.clone(), cache);
    cache.insert(state, output.clone());
    output
}

// returns the lowest amount of mana to win, or none if its impossible to win
fn player_turn(mut state: GameState, cache: &mut HashMap<GameState, Option<i32>>) -> Option<i32> {
    if state.hard {
        state.player_health -= 1;
    }

    // if the boss kills the player, we can't win, so return None
    if state.player_health <= 0 {
        return None;
    }

    state.apply_effects();

    if state.boss_health <= 0 {
        return Some(state.player_mana_spent);
    }

    [
        magic_missile(state.clone(), cache),
        drain(state.clone(), cache),
        shield(state.clone(), cache),
        poison(state.clone(), cache),
        recharge(state.clone(), cache),
    ]
    .into_iter()
    .flatten()
    .min()
}

fn boss_turn(mut state: GameState, cache: &mut HashMap<GameState, Option<i32>>) -> Option<i32> {
    state.apply_effects();

    // if the player just killed the boss, the player won.
    if state.boss_health <= 0 {
        return Some(state.player_mana_spent);
    }

    // the shield effect is the only thing that affects armor
    let player_armor = tern!(state.shield_timer > 0, 7, 0);

    let damage_difference = state.boss_damage - player_armor;
    let damage_dealt = damage_difference.max(1);
    state.player_health -= damage_dealt;
    cached(state, player_turn, cache)
}

fn part1(input: &str) -> u32 {
    let (boss_health, boss_damage) = input
        .lines()
        .map(|l| l.split_once(": ").unwrap().1.parse().unwrap())
        .collect_tuple()
        .unwrap();
    let state = GameState {
        player_health: 50,
        player_mana: 500,
        player_mana_spent: 0,
        boss_health,
        boss_damage,
        shield_timer: 0,
        poison_timer: 0,
        recharge_timer: 0,
        hard: false,
    };
    let mut cache = HashMap::new();
    cached(state, player_turn, &mut cache).unwrap() as u32
}

fn part2(input: &str) -> u32 {
    let (boss_health, boss_damage) = input
        .lines()
        .map(|l| l.split_once(": ").unwrap().1.parse().unwrap())
        .collect_tuple()
        .unwrap();
    let state = GameState {
        player_health: 50,
        player_mana: 500,
        player_mana_spent: 0,
        boss_health,
        boss_damage,
        shield_timer: 0,
        poison_timer: 0,
        recharge_timer: 0,
        hard: true,
    };
    let mut cache = HashMap::new();
    cached(state, player_turn, &mut cache).unwrap() as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day() {
        let input = include_str!("./input.txt").trim();
        assert_eq!(part1(input), 1824);
        assert_eq!(part2(input), 1937);
    }
}
