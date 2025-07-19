fn main() {
    let input = include_str!("./input.txt").trim();
    println!("{}", part1(input));
    println!("{}", part2(input));
}

#[derive(Debug, Default)]
struct CubeSet {
    red: u32,
    green: u32,
    blue: u32,
}

fn parse_cube_set(s: &str) -> CubeSet {
    s.split(", ").map(|s| s.split_once(' ').unwrap()).fold(
        CubeSet::default(),
        |mut acc, (count_str, color)| {
            let count = count_str.parse::<u32>().unwrap();
            match color {
                "red" => acc.red += count,
                "green" => acc.green += count,
                "blue" => acc.blue += count,
                _ => unreachable!(),
            }
            acc
        },
    )
}

struct Game {
    game_index: u32,
    sets: Vec<CubeSet>,
}

// Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green -> Game {game_index: 1, sets: [...]}
fn parse_game(s: &str) -> Game {
    let (game_id, sets_str) = s.split_once(": ").unwrap();
    let game_index = game_id.split_once(' ').unwrap().1.parse().unwrap();
    let sets = sets_str.split("; ").map(parse_cube_set).collect();
    Game { game_index, sets }
}

fn part1(input: &str) -> u32 {
    input
        .lines()
        .map(parse_game)
        .filter(|g| {
            g.sets
                .iter()
                .all(|set| set.red <= 12 && set.green <= 13 && set.blue <= 14)
        })
        .map(|g| g.game_index)
        .sum()
}

fn part2(input: &str) -> u32 {
    input
        .lines()
        .map(parse_game)
        .map(|g| {
            g.sets.iter().map(|set| set.red).max().unwrap_or(0)
                * g.sets.iter().map(|set| set.green).max().unwrap_or(0)
                * g.sets.iter().map(|set| set.blue).max().unwrap_or(0)
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day() {
        let input = include_str!("./input.txt").trim();
        assert_eq!(part1(input), 2204);
        assert_eq!(part2(input), 71036);
    }
}
