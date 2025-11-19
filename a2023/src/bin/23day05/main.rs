use lib::{Range, Ranged, StringTools, itertools::Itertools, pop_loop};

fn main() {
    let input = include_str!("./input.txt").trim();
    println!("{}", part1(input));
    println!("{}", part2(input));
}

struct AttributeMap {
    target_range: Range<i64>,
    offset: i64,
}

fn parse_attribute_map(s: &str) -> AttributeMap {
    let (dest_start, src_start, length) = s
        .split_whitespace()
        .map(|d| d.parse().unwrap())
        .collect_tuple()
        .unwrap();

    AttributeMap {
        target_range: Range::new_by_len(src_start, length),
        offset: dest_start - src_start,
    }
}

struct Almanac {
    seeds: Vec<i64>,
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

fn map_attribute(attribute: i64, map: &[AttributeMap]) -> i64 {
    map.iter()
        .find(|map_line| map_line.target_range.contains(attribute))
        .map_or(attribute, |map_line| attribute + map_line.offset)
}

fn part1(input: &str) -> i64 {
    let almanac = parse_almanac(input);

    almanac
        .seeds
        .into_iter()
        .map(|seed| {
            almanac
                .maps
                .iter()
                .fold(seed, |attribute, map| map_attribute(attribute, map))
        })
        .min()
        .unwrap()
}

fn part2(input: &str) -> i64 {
    let almanac: Almanac = parse_almanac(input);

    let mut to_process = almanac
        .seeds
        .chunks(2)
        .map(|c| Range::new_by_len(c[0], c[1]))
        .collect_vec();

    let mut output = Vec::new();

    for map in almanac.maps {
        pop_loop!(to_process, attr_range, {
            let found = map
                .iter()
                .find(|map_line| map_line.target_range.overlaps(attr_range));

            let outputted = if let Some(map_line) = found {
                to_process.extend(attr_range.remove(map_line.target_range));
                attr_range.intersection(map_line.target_range).unwrap() + map_line.offset
            } else {
                attr_range
            };

            output.push(outputted);
        });

        to_process = output;
        output = Vec::new();
    }

    to_process.into_iter().map(Range::start).min().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day() {
        let input = include_str!("./input.txt").trim();
        assert_eq!(part1(input), 111627841);
        assert_eq!(part2(input), 69323688);
    }
}
