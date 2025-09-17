use lib::{Grid, IteratorExt, itertools::Itertools};

fn main() {
    let input = include_str!("./input.txt").trim();
    println!("{}", part1(input));
    println!("{}", part2(input));
}

fn indices_without_galaxies(g: &[Vec<char>]) -> Vec<usize> {
    g.iter()
        .enumerate()
        .filter(|(_, row)| row.iter().all(|&c| c == '.'))
        .map(|(i, _)| i)
        .collect()
}

fn count_extra_spaces(spaces_without_galaxies: &[usize], p1_d: usize, p2_d: usize) -> usize {
    spaces_without_galaxies
        .iter()
        .count_where(|r| (p1_d..p2_d).contains(r) || (p2_d..p1_d).contains(r))
}

fn galaxy_distances(grid: &Grid<char>, expansion_rate: usize) -> u64 {
    let rows_without_galaxies = indices_without_galaxies(grid.rows());
    let cols_without_galaxies = indices_without_galaxies(grid.cols());

    grid.find_all(&'#')
        .tuple_combinations()
        .map(|(a, b)| {
            (a.manhattan_dist(b)
                + count_extra_spaces(&rows_without_galaxies, a.y, b.y) * (expansion_rate - 1)
                + count_extra_spaces(&cols_without_galaxies, a.x, b.x) * (expansion_rate - 1))
                as u64
        })
        .sum()
}

fn part1(input: &str) -> u64 {
    let grid = Grid::from_chars_transpose(input).unwrap();
    galaxy_distances(&grid, 2)
}

fn part2(input: &str) -> u64 {
    let grid = Grid::from_chars_transpose(input).unwrap();
    galaxy_distances(&grid, 1000000)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day() {
        let input = include_str!("./input.txt").trim();
        assert_eq!(part1(input), 10165598);
        assert_eq!(part2(input), 678728808158);
    }
}
