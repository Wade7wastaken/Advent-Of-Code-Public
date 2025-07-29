use std::collections::{HashMap, VecDeque};

use lib::itertools::Itertools;

fn main() {
    let input = include_str!("./input.txt").trim();
    println!("{}", part1(input));
    println!("{}", part2(input));
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum IpcMessage {
    Send(i64),
    Receive,
}

#[derive(Debug, Clone, Default)]
struct Computer<'a> {
    r: HashMap<&'a str, i64>,
    pc: usize,
    sound: i64,
    queue: VecDeque<i64>,
    send_counter: u32,
}

impl<'a> Computer<'a> {
    fn new_dual(id: i64) -> Self {
        Self {
            r: HashMap::from([("p", id)]),
            ..Default::default()
        }
    }
    fn get_number(&self, x: &str) -> i64 {
        x.parse::<i64>().unwrap_or_else(|_| *self.r.get(x).unwrap())
    }
    fn get_register(&mut self, x: &'a str) -> &mut i64 {
        self.r.entry(x).or_insert(0)
    }
    fn run_sound(&mut self, instrs: &[&'a str]) -> i64 {
        loop {
            let instr = instrs[self.pc];
            self.pc += 1;
            match instr.split_ascii_whitespace().collect_vec()[..] {
                ["snd", x] => {
                    self.sound = self.get_number(x);
                }
                ["set", x, y] => *self.get_register(x) = self.get_number(y),
                ["add", x, y] => *self.get_register(x) += self.get_number(y),
                ["mul", x, y] => *self.get_register(x) *= self.get_number(y),
                ["mod", x, y] => *self.get_register(x) %= self.get_number(y),
                ["rcv", x] => {
                    if self.get_number(x) != 0 {
                        return self.sound;
                    }
                }
                ["jgz", x, y] => {
                    if self.get_number(x) > 0 {
                        self.pc -= 1;
                        self.pc = (self.pc as i64 + self.get_number(y)) as usize;
                    }
                }
                _ => panic!("unknown instruction {instr}"),
            }
        }
    }

    fn run_dual(&mut self, instrs: &[&'a str]) -> IpcMessage {
        loop {
            let instr = instrs[self.pc];
            self.pc += 1;
            match instr.split_ascii_whitespace().collect_vec()[..] {
                ["snd", x] => {
                    self.send_counter += 1;
                    return IpcMessage::Send(self.get_number(x));
                }
                ["set", x, y] => *self.get_register(x) = self.get_number(y),
                ["add", x, y] => *self.get_register(x) += self.get_number(y),
                ["mul", x, y] => *self.get_register(x) *= self.get_number(y),
                ["mod", x, y] => *self.get_register(x) %= self.get_number(y),
                ["rcv", x] => {
                    if let Some(popped) = self.queue.pop_front() {
                        *self.get_register(x) = popped;
                    } else {
                        self.pc -= 1;
                        return IpcMessage::Receive;
                    }
                }
                ["jgz", x, y] => {
                    if self.get_number(x) > 0 {
                        self.pc -= 1;
                        self.pc = (self.pc as i64 + self.get_number(y)) as usize;
                    }
                }
                _ => panic!("unknown instruction {instr}"),
            }
        }
    }
}

fn part1(input: &str) -> i64 {
    let mut computer = Computer::default();
    computer.run_sound(&input.lines().collect_vec())
}

fn part2(input: &str) -> u32 {
    let instrs = input.lines().collect_vec();
    let mut a = Computer::new_dual(0);
    let mut b = Computer::new_dual(1);

    loop {
        let a_res = a.run_dual(&instrs);
        let b_res = b.run_dual(&instrs);

        if a_res == IpcMessage::Receive && b_res == IpcMessage::Receive {
            return b.send_counter;
        }
        if let IpcMessage::Send(s) = a_res {
            b.queue.push_back(s);
        }
        if let IpcMessage::Send(s) = b_res {
            a.queue.push_back(s);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day() {
        let input = include_str!("./input.txt").trim();
        assert_eq!(part1(input), 4601);
        assert_eq!(part2(input), 6858);
    }
}
