use std::cmp::Reverse;

use lib::itertools::Itertools;

fn main() {
    let input = include_str!("./input.txt").trim();
    println!("{}", part1(input));
    println!("{}", part2(input));
}

struct Reindeer {
    speed: u32,
    fly_time: u32,
    rest_time: u32,

    distance: u32,
    fly_timer: u32,
    rest_timer: u32,

    points: u32,
}

impl Reindeer {
    fn new(speed: u32, fly_time: u32, rest_time: u32) -> Self {
        Self {
            speed,
            fly_time,
            rest_time,
            distance: 0,
            fly_timer: fly_time,
            rest_timer: 0,
            points: 0,
        }
    }

    fn tick(&mut self) {
        if self.fly_timer == 0 {
            // resting
            self.rest_timer -= 1;
            if self.rest_timer == 0 {
                self.fly_timer = self.fly_time;
            }
        } else {
            // flying
            self.fly_timer -= 1;
            self.distance += self.speed;
            if self.fly_timer == 0 {
                self.rest_timer = self.rest_time;
            }
        }
    }
}

fn part1(input: &str) -> u32 {
    input
        .lines()
        .map(|l| l.split_ascii_whitespace().collect_vec())
        .map(|strs| {
            Reindeer::new(
                strs[3].parse().unwrap(),
                strs[6].parse().unwrap(),
                strs[13].parse().unwrap(),
            )
        })
        .update(|deer| {
            for _ in 0..2503 {
                deer.tick();
            }
        })
        .max_by_key(|d| d.distance)
        .unwrap()
        .distance
}

fn part2(input: &str) -> u32 {
    let mut deer = input
        .lines()
        .map(|l| l.split_ascii_whitespace().collect_vec())
        .map(|strs| {
            Reindeer::new(
                strs[3].parse().unwrap(),
                strs[6].parse().unwrap(),
                strs[13].parse().unwrap(),
            )
        }).collect_vec();

        for _ in 0..2503 {
            for d in &mut deer {
                d.tick();
            }
            deer.sort_by_key(|d| Reverse(d.distance));
            let max = deer[0].distance;
            for d in &mut deer {
                if d.distance == max {
                    d.points += 1;
                } else {
                    break;
                }
            }
        }

        deer.into_iter().max_by_key(|d| d.points).unwrap().points

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day() {
        let input = include_str!("./input.txt").trim();
        assert_eq!(part1(input), 2640);
        assert_eq!(part2(input), 1102);
    }
}
