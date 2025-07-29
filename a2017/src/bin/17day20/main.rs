use std::{collections::HashSet, hash::Hash, ops::AddAssign};

use lib::itertools::Itertools;

fn main() {
    let input = include_str!("./input.txt").trim();
    println!("{}", part1(input));
    println!("{}", part2(input));
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Vec3 {
    x: i64,
    y: i64,
    z: i64,
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl Vec3 {
    fn len(self) -> i64 {
        let Self { x, y, z } = self;
        x.abs() + y.abs() + z.abs()
    }
}

fn parse_vec3(s: &str) -> Vec3 {
    let (x, y, z) = s
        .strip_prefix('<')
        .unwrap()
        .strip_suffix('>')
        .expect(s)
        .split(',')
        .map(|n| n.trim().parse().unwrap())
        .collect_tuple()
        .unwrap();
    Vec3 { x, y, z }
}

#[derive(Debug, Clone, Eq)]
struct Particle {
    pos: Vec3,
    vel: Vec3,
    acc: Vec3,
}

impl PartialEq for Particle {
    fn eq(&self, other: &Self) -> bool {
        self.pos == other.pos
    }
}

impl Hash for Particle {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.pos.hash(state);
    }
}

impl Particle {
    fn tick(&mut self) {
        self.vel += self.acc;
        self.pos += self.vel;
    }
}

fn parse_particle(s: &str) -> Particle {
    let (pos, vel, acc) = s
        .split(", ")
        .map(|v| parse_vec3(&v.trim()[2..]))
        .collect_tuple()
        .unwrap();
    Particle { pos, vel, acc }
}

fn part1(input: &str) -> u32 {
    let mut particles = input.lines().map(parse_particle).enumerate().collect_vec();

    for _ in 0..100000 {
        for p in &mut particles {
            p.1.tick();
        }
    }
    particles.iter().min_by_key(|p| p.1.pos.len()).unwrap().0 as u32
}

fn part2(input: &str) -> u32 {
    let mut particles = input.lines().map(parse_particle).collect::<HashSet<_>>();

    for _ in 0..1000 {
        let mut new_particles = HashSet::new();
        let mut to_remove = vec![];
        for p in particles {
            let mut ticked = p.clone();
            ticked.tick();
            if !new_particles.insert(ticked.clone()) {
                to_remove.push(ticked);
            }
        }
        for p in to_remove {
            new_particles.remove(&p);
        }
        particles = new_particles;
    }

    particles.len() as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day() {
        let input = include_str!("./input.txt").trim();
        assert_eq!(part1(input), 170);
        assert_eq!(part2(input), 571);
    }
}
