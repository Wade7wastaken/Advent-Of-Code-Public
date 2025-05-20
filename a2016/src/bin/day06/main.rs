use lib::{CollectString, Grid, itertools::Itertools};

fn main() {
    let input = include_str!("./input.txt").trim();
    println!("{}", part1(input));
    println!("{}", part2(input));
}

fn part1(input: &str) -> String {
    Grid::from_bytes(input)
        .unwrap()
        .into_cols_iter()
        .map(|col| {
            col.into_iter()
                .counts()
                .into_iter()
                .max_by_key(|x| x.1)
                .unwrap()
                .0
        })
        .collect_string()
}

fn part2(input: &str) -> String {
    Grid::from_bytes(input)
        .unwrap()
        .into_cols_iter()
        .map(|col| {
            col.into_iter()
                .counts()
                .into_iter()
                .min_by_key(|x| x.1)
                .unwrap()
                .0
        })
        .collect_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day() {
        let input = include_str!("./input.txt").trim();
        assert_eq!(part1(input), "dzqckwsd");
        assert_eq!(part2(input), "lragovly");
    }
}
