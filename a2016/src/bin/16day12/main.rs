use lib::itertools::Itertools;

fn main() {
    let input = include_str!("./input.txt").trim();
    println!("{}", part1(input));
    println!("{}", part2(input));
}

#[derive(Debug, Clone, Copy)]
enum Register {
    A,
    B,
    C,
    D,
    Immediate(i32),
}

impl Register {
    fn new(r: &str) -> Self {
        match r {
            "a" => Register::A,
            "b" => Register::B,
            "c" => Register::C,
            "d" => Register::D,
            _ => Register::Immediate(r.parse().unwrap()),
        }
    }
}

#[derive(Debug, Clone)]
enum Instr {
    Cpy(Register, Register),
    Inc(Register),
    Dec(Register),
    Jnz(Register, Register),
}

impl Instr {
    fn new(line: &str) -> Self {
        match line.split_ascii_whitespace().collect_vec()[..] {
            ["cpy", x, y] => Instr::Cpy(Register::new(x), Register::new(y)),
            ["inc", x] => Instr::Inc(Register::new(x)),
            ["dec", x] => Instr::Dec(Register::new(x)),
            ["jnz", x, y] => Instr::Jnz(Register::new(x), Register::new(y)),
            _ => panic!(),
        }
    }
}

struct Computer {
    a: i32,
    b: i32,
    c: i32,
    d: i32,
    pc: usize,
}

impl Computer {
    const fn new(a: i32, b: i32, c: i32, d: i32) -> Self {
        Self { a, b, c, d, pc: 0 }
    }

    const fn reg(&mut self, r: Register) -> Option<&mut i32> {
        match r {
            Register::A => Some(&mut self.a),
            Register::B => Some(&mut self.b),
            Register::C => Some(&mut self.c),
            Register::D => Some(&mut self.d),
            Register::Immediate(_) => None,
        }
    }

    const fn value(&self, r: Register) -> i32 {
        match r {
            Register::A => self.a,
            Register::B => self.b,
            Register::C => self.c,
            Register::D => self.d,
            Register::Immediate(i) => i,
        }
    }

    fn execute(&mut self, instrs: &[Instr]) {
        while let Some(instr) = instrs.get(self.pc).cloned() {
            self.pc += 1;
            match instr {
                Instr::Cpy(x, y) => *self.reg(y).unwrap() = self.value(x),
                Instr::Inc(x) => *self.reg(x).unwrap() += 1,
                Instr::Dec(x) => *self.reg(x).unwrap() -= 1,
                Instr::Jnz(x, y) => {
                    if self.value(x) != 0 {
                        self.pc = (self.pc as i32 + self.value(y) - 1) as usize;
                    }
                }
            }
        }
    }
}

fn part1(input: &str) -> i32 {
    let mut computer = Computer::new(0, 0, 0, 0);
    let instrs = input.lines().map(Instr::new).collect_vec();
    computer.execute(&instrs);
    computer.a
}

fn part2(input: &str) -> i32 {
    let mut computer = Computer::new(0, 0, 1, 0);
    let instrs = input.lines().map(Instr::new).collect_vec();
    computer.execute(&instrs);
    computer.a
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day() {
        let input = include_str!("./input.txt").trim();
        assert_eq!(part1(input), 318003);
        assert_eq!(part2(input), 9227657);
    }
}
