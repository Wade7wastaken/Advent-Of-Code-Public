use lib::{itertools::Itertools, StringTools};

fn main() {
    let input = include_str!("./input.txt").trim();
    println!("{}", part1(input));
    println!("{}", part2(input));
}

struct AttributeMap {
    dest_start: u32,
    src_start: u32,
    length: u32,
}

fn parse_attribute_map(s: &str) -> AttributeMap {
    let (dest_start, src_start, length) = s
        .split_whitespace()
        .map(|d| d.parse().unwrap())
        .collect_tuple()
        .unwrap();

    AttributeMap {
        dest_start,
        src_start,
        length,
    }
}

struct Almanac {
    seeds: Vec<u32>,
    maps: Vec<Vec<AttributeMap>>,
}

fn parse_almanac(s: &str) -> Almanac {
    let mut paragraphs = s.paragraphs();
    let seeds_line = paragraphs.next().unwrap();
    let seeds = seeds_line
        .split_once(": ")
        .unwrap()
        .1
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let maps = paragraphs
        .map(|map| map.lines().skip(1).map(parse_attribute_map).collect())
        .collect();

    Almanac { seeds, maps }
}

fn map_attribute(attribute: u32, map: &Vec<AttributeMap>) -> u32 {
    for map_line in map {
        if attribute >= map_line.src_start {
            let offset = attribute - map_line.src_start;
            if offset < map_line.length {
                return map_line.dest_start + offset;
            }
        }
    }
    attribute
}

fn part1(input: &str) -> u32 {
    let almanac = parse_almanac(input);

    almanac
        .seeds
        .into_iter()
        .map(|seed| almanac.maps.iter().fold(seed, map_attribute))
        .min()
        .unwrap()
}

fn part2(input: &str) -> u32 {
    let almanac: Almanac = parse_almanac(input);

    almanac
        .seeds
        .chunks(2)
        .flat_map(|seed_range| {
            let result = (seed_range[0]..seed_range[0] + seed_range[1])
                .map(|seed| almanac.maps.iter().fold(seed, map_attribute));
            println!("Done with seed map {seed_range:?}");
            result
        })
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day() {
        let input = include_str!("./input.txt").trim();
        assert_eq!(part1(input), 111627841);
        // assert_eq!(part2(input), 69323688);
    }
}
