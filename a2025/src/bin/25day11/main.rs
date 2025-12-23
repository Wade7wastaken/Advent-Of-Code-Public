use std::collections::HashMap;

fn main() {
    let input = include_str!("./input.txt").trim();
    println!("{}", part1(input));
    println!("{}", part2(input));
}

fn parse_connections(input: &str) -> HashMap<&str, Vec<&str>> {
    input
        .lines()
        .map(|device| {
            let (name, outputs) = device.split_once(": ").unwrap();
            (name, outputs.split_ascii_whitespace().collect())
        })
        .collect()
}

fn traverse<'a>(
    from: &'a str,
    to: &'a str,
    connections: &HashMap<&'a str, Vec<&'a str>>,
    cache: &mut HashMap<&'a str, u64>,
) -> u64 {
    if let Some(cached) = cache.get(from) {
        return *cached;
    }
    if from == to {
        return 1;
    }
    if from == "out" {
        return 0;
    }

    let sum = connections
        .get(from)
        .unwrap()
        .iter()
        .map(|c| traverse(c, to, connections, cache))
        .sum();
    cache.insert(from, sum);
    sum
}

fn num_paths<'a>(from: &'a str, to: &'a str, connections: &HashMap<&'a str, Vec<&'a str>>) -> u64 {
    let mut cache = HashMap::new();
    traverse(from, to, connections, &mut cache)
}

fn part1(input: &str) -> u64 {
    num_paths("you", "out", &parse_connections(input))
}

fn part2(input: &str) -> u64 {
    let connections = parse_connections(input);

    let fft_first = num_paths("svr", "fft", &connections)
        * num_paths("fft", "dac", &connections)
        * num_paths("dac", "out", &connections);
    let svr_first = num_paths("svr", "dac", &connections)
        * num_paths("dac", "fft", &connections)
        * num_paths("fft", "out", &connections);

    fft_first + svr_first
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day() {
        let input = include_str!("./input.txt").trim();
        assert_eq!(part1(input), 764);
        assert_eq!(part2(input), 462444153119850);
    }
}
