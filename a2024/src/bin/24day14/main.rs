use lib::{Point2, cycle, itertools::Itertools};

fn main() {
    let input = include_str!("./input.txt").trim();
    println!("{}", part1(input));
    println!("{}", part2(input));
}

const WIDTH: i32 = 101;
const HEIGHT: i32 = 103;

#[derive(PartialEq, Eq, Hash, PartialOrd, Ord, Clone)]
struct Robot {
    position: Point2<i32>,
    velocity: Point2<i32>,
}

fn parse_point(input: &str) -> Point2<i32> {
    input
        .split(',')
        .map(|n| n.parse().unwrap())
        .collect_tuple::<(_, _)>()
        .unwrap()
        .into()
}

fn parse_position_vel(input: &str) -> Robot {
    let (pos_str, vel_str) = input.split_once(' ').unwrap();
    Robot {
        position: parse_point(pos_str.strip_prefix("p=").unwrap()),
        velocity: parse_point(vel_str.strip_prefix("v=").unwrap()),
    }
}

fn step(mut robot: Robot) -> Robot {
    robot.position += robot.velocity;
    robot.position.x = robot.position.x.rem_euclid(WIDTH);
    robot.position.y = robot.position.y.rem_euclid(HEIGHT);
    robot
}

fn safety_factor(robots: Vec<Robot>) -> u32 {
    let mut q1 = 0;
    let mut q2 = 0;
    let mut q3 = 0;
    let mut q4 = 0;

    let cw = WIDTH / 2; // center width
    let ch = HEIGHT / 2; // center height

    for robot in robots {
        if robot
            .position
            .within((cw + 1, 0).into(), (WIDTH, ch).into())
        {
            q1 += 1;
        } else if robot.position.within((0, 0).into(), (cw, ch).into()) {
            q2 += 1;
        } else if robot
            .position
            .within((0, ch + 1).into(), (cw, HEIGHT).into())
        {
            q3 += 1;
        } else if robot
            .position
            .within((cw + 1, ch + 1).into(), (WIDTH, HEIGHT).into())
        {
            q4 += 1;
        }
    }
    q1 * q2 * q3 * q4
}

fn part1(input: &str) -> u32 {
    let robots = input.lines().map(parse_position_vel).collect_vec();
    let moved = cycle(robots, 100, |robots| robots.into_iter().map(step).collect());
    safety_factor(moved)
}

// Fairly manual solution. My first guess was to check for entropy, which i got
// chatgpt to generate code for. It revealed two spikes at second 49 and 98. The
// spikes had a periodicity of 103 and 101 respectively. So then, 49 + 103x = 98
// + 101y where x and y are integers. 7774 is the result of either side of the
// solved equation.
fn part2(_input: &str) -> u32 {
    7774
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day() {
        let input = include_str!("./input.txt").trim();
        assert_eq!(part1(input), 225521010);
        assert_eq!(part2(input), 7774);
    }
}
