use lib::itertools::Itertools;

fn main() {
    let input = include_str!("./input.txt").trim();
    println!("{}", part1(input));
    // println!("{}", part2(input));
}

#[derive(Debug, Clone, Copy)]
enum Register {
    A,
    B,
    C,
    D,
    Immediate(i32),
}

fn parse_register(r: &str) -> Register {
    match r {
        "a" => Register::A,
        "b" => Register::B,
        "c" => Register::C,
        "d" => Register::D,
        _ => Register::Immediate(r.parse().unwrap()),
    }
}

#[derive(Debug, Clone)]
enum Instr {
    Cpy(Register, Register),
    Inc(Register),
    Dec(Register),
    Jnz(Register, Register),
    Tgl(Register),
}

impl Instr {
    fn toggle(&mut self) {
        *self = match *self {
            Instr::Cpy(x, y) => Instr::Jnz(x, y),
            Instr::Inc(x) => Instr::Dec(x),
            Instr::Dec(x) => Instr::Inc(x),
            Instr::Jnz(x, y) => Instr::Cpy(x, y),
            Instr::Tgl(x) => Instr::Inc(x),
        }
    }
}

fn parse_instr(line: &str) -> Instr {
    match line.split_ascii_whitespace().collect_vec()[..] {
        ["cpy", x, y] => Instr::Cpy(parse_register(x), parse_register(y)),
        ["inc", x] => Instr::Inc(parse_register(x)),
        ["dec", x] => Instr::Dec(parse_register(x)),
        ["jnz", x, y] => Instr::Jnz(parse_register(x), parse_register(y)),
        ["tgl", x] => Instr::Tgl(parse_register(x)),
        _ => panic!(),
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
    fn new(a: i32, b: i32, c: i32, d: i32) -> Self {
        Self { a, b, c, d, pc: 0 }
    }

    fn reg(&mut self, r: Register) -> Option<&mut i32> {
        match r {
            Register::A => Some(&mut self.a),
            Register::B => Some(&mut self.b),
            Register::C => Some(&mut self.c),
            Register::D => Some(&mut self.d),
            Register::Immediate(_) => None,
        }
    }

    fn value(&self, r: Register) -> i32 {
        match r {
            Register::A => self.a,
            Register::B => self.b,
            Register::C => self.c,
            Register::D => self.d,
            Register::Immediate(i) => i,
        }
    }

    fn execute(&mut self, instrs: &mut [Instr]) {
        while let Some(instr) = instrs.get(self.pc).cloned() {
            self.pc += 1;
            match instr {
                Instr::Cpy(x, y) => {
                    let x_value = self.value(x);
                    if let Some(a) = self.reg(y) {
                        *a = x_value;
                    }
                }
                Instr::Inc(x) => *self.reg(x).unwrap() += 1,
                Instr::Dec(x) => *self.reg(x).unwrap() -= 1,
                Instr::Jnz(x, y) => {
                    if self.value(x) != 0 {
                        self.pc = (self.pc as i32 + self.value(y) - 1) as usize;
                    }
                }
                Instr::Tgl(x) => {
                    let toggle_pos = self.pc as i32 + self.value(x) - 1;
                    if let Some(toggle_instr) = instrs.get_mut(toggle_pos as usize) {
                        toggle_instr.toggle();
                    }
                }
            }
        }
    }
}

fn part1(input: &str) -> i32 {
    let mut computer = Computer::new(12, 0, 0, 0);
    let mut instrs = input.lines().map(parse_instr).collect_vec();
    computer.execute(&mut instrs);
    computer.a
}

fn part2(input: &str) -> u32 {
    todo!();
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
