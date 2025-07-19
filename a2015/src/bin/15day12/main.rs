use lib::serde_json::{self, Value};

fn main() {
    let input = include_str!("./input.txt").trim();
    println!("{}", part1(input));
    println!("{}", part2(input));
}

fn find_numbers(data: Value) -> i64 {
    match data {
        Value::Number(n) => n.as_i64().unwrap(),
        Value::Array(arr) => arr.into_iter().map(find_numbers).sum(),
        Value::Object(obj) => obj.into_values().map(find_numbers).sum(),
        _ => 0
    }
}

fn part1(input: &str) -> u32 {
    find_numbers(serde_json::from_str(input).unwrap()) as u32
}

fn find_numbers_without_red(data: Value) -> i64 {
    match data {
        Value::Number(n) => n.as_i64().unwrap(),
        Value::Array(arr) => arr.into_iter().map(find_numbers_without_red).sum(),
        Value::Object(obj) => {
            if obj.values().any(|val| val == &Value::String("red".to_string())) {
                return 0;
            }
            obj.into_values().map(find_numbers_without_red).sum()
        },
        _ => 0
    }
}

fn part2(input: &str) -> u32 {
    find_numbers_without_red(serde_json::from_str(input).unwrap()) as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day() {
        let input = include_str!("./input.txt").trim();
        assert_eq!(part1(input), 156366);
        assert_eq!(part2(input), 96852);
    }
}

