use std::collections::HashMap;

use lib::itertools::Itertools;

fn main() {
    let input = include_str!("./input.txt").trim();
    // println!("{}", part1(input));
    println!("{}", part2(input));
}

#[derive(Debug, Clone, Default)]
struct Computer<'a> {
    r: HashMap<&'a str, i64>,
    pc: usize,
    mul_counter: u64,
}

impl<'a> Computer<'a> {
    fn get_number(&mut self, x: &'a str) -> i64 {
        x.parse::<i64>()
            .unwrap_or_else(|_| *self.r.entry(x).or_insert(0))
    }
    fn get_register(&mut self, x: &'a str) -> &mut i64 {
        self.r.entry(x).or_insert(0)
    }
    fn run(&mut self, instrs: &[&'a str]) -> u64 {
        loop {
            let instr = instrs.get(self.pc);
            if instr.is_none() {
                return self.mul_counter;
            }
            let instr = instr.unwrap();
            self.pc += 1;
            match instr.split_ascii_whitespace().collect_vec()[..] {
                ["set", x, y] => *self.get_register(x) = self.get_number(y),
                ["sub", x, y] => *self.get_register(x) -= self.get_number(y),
                ["mul", x, y] => {
                    *self.get_register(x) *= self.get_number(y);
                    self.mul_counter += 1;
                }
                ["jnz", x, y] => {
                    if self.get_number(x) != 0 {
                        self.pc -= 1;
                        self.pc = (self.pc as i64 + self.get_number(y)) as usize;
                    }
                }
                _ => panic!("unknown instruction {instr}"),
            }
        }
    }
}

fn part1(input: &str) -> u64 {
    let mut c = Computer::default();
    c.run(&input.lines().collect_vec())
}

fn part2(input: &str) -> i64 {
    let mut c = Computer::default();
    c.r.insert("a", 1);
    c.run(&input.lines().collect_vec());
    *c.r.get("h").unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day() {
        let input = include_str!("./input.txt").trim();
        // assert_eq!(part1(input), todo!());
        // assert_eq!(part2(input), todo!());
    }
}
