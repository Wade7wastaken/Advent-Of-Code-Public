use lib::{itertools::Itertools, Grid, Point2, Range, Ranged};

fn main() {
    let input = include_str!("./input.txt").trim();
    println!("{}", part1(input));
    println!("{}", part2(input));
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum ClaimCell {
    NotClaimed,
    ClaimedOnce,
    ClaimedTwice,
}

impl ClaimCell {
    fn claim(&mut self) {
        *self = match *self {
            Self::NotClaimed => Self::ClaimedOnce,
            Self::ClaimedOnce => Self::ClaimedTwice,
            Self::ClaimedTwice => Self::ClaimedTwice,
        }
    }
}

#[derive(Debug, PartialEq)]
struct Claim {
    start: Point2<usize>,
    size: Point2<usize>,
    id: u32,
}

impl Claim {
    fn x_range(&self) -> Range<usize> {
        Range::new(self.start.x, self.start.x + self.size.x)
    }
    fn y_range(&self) -> Range<usize> {
        Range::new(self.start.y, self.start.y + self.size.y)
    }
}

fn parse_claim(input: &str) -> Claim {
    let (id_str, data_str) = input.split_once(" @ ").unwrap();
    let (start_str, size_str) = data_str.split_once(": ").unwrap();
    Claim {
        start: start_str
            .split(',')
            .map(|n| n.parse().unwrap())
            .collect_tuple::<(_, _)>()
            .unwrap()
            .into(),
        size: size_str
            .split('x')
            .map(|n| n.parse().unwrap())
            .collect_tuple::<(_, _)>()
            .unwrap()
            .into(),
        id: id_str.strip_prefix('#').unwrap().trim().parse().unwrap(),
    }
}

fn part1(input: &str) -> u32 {
    let mut grid = Grid::new_filled(ClaimCell::NotClaimed, 1000, 1000);

    for claim in input.lines().map(parse_claim) {
        for y in claim.start.y..claim.start.y + claim.size.y {
            for x in claim.start.x..claim.start.x + claim.size.x {
                grid.get_mut((x, y)).unwrap().claim();
            }
        }
    }

    grid.count(&ClaimCell::ClaimedTwice) as u32
}

fn part2(input: &str) -> u32 {
    let claims = input.lines().map(parse_claim).collect_vec();

    'outer: for test_claim in &claims {
        for other in claims.iter().filter(|c| **c != *test_claim) {
            if test_claim.x_range().overlaps(other.x_range())
                && test_claim.y_range().overlaps(other.y_range())
            {
                continue 'outer;
            }
        }
        return test_claim.id;
    }

    panic!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day() {
        let input = include_str!("./input.txt").trim();
        assert_eq!(part1(input), 109143);
        assert_eq!(part2(input), 506);
    }
}
