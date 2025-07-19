use lib::num::Integer;

fn main() {
    let input = include_str!("./input.txt").trim();
    println!("{}", part1(input));
    println!("{}", part2(input));
}

fn sum_of_factors(n: u32) -> u32 {
    let mut sum = n + 1;
    for i in 2..n.isqrt() {
        let (div, m) = n.div_mod_floor(&i);
        if m == 0 {
            sum += i;
            if i != div {
                sum += div;
            }
        }
    }

    sum
}

fn part1(input: &str) -> u32 {
    let min_result = input.parse::<u32>().unwrap() / 10;
    let mut house_number = 1;
    loop {
        let sum = sum_of_factors(house_number);
        if sum >= min_result {
            break house_number;
        }
        house_number += 1;
    }
}

fn sum_of_factors_50(n: u32) -> u32 {
    let mut sum = 0;
    for i in 1..n.isqrt() {
        let (div, m) = n.div_mod_floor(&i);
        if m != 0 {
            continue;
        }

        if i == div {
            if i <= 50 {
                sum += i;
            }
            continue;
        }

        if div <= 50 {
            sum += i;
        }
        if i <= 50 {
            sum += div;
        }
    }

    sum
}

fn part2(input: &str) -> u32 {
    let min_result = input.parse::<u32>().unwrap();
    let mut house_number = 1;
    loop {
        let sum = sum_of_factors_50(house_number) * 11;
        if sum >= min_result {
            break house_number;
        }
        house_number += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day() {
        let input = include_str!("./input.txt").trim();
        assert_eq!(part1(input), 831600);
        assert_eq!(part2(input), 884520);
    }
}
