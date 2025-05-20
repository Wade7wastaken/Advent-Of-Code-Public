use lib::itertools::Itertools;

fn main() {
    let input = include_str!("./input.txt").trim();
    println!("{}", part1(input));
    println!("{}", part2(input));
}

struct Computer {
    a: u32,
    b: u32,
    ip: i32,
}

impl Computer {
    fn new() -> Self {
        Computer { a: 0, b: 0, ip: 0 }
    }

    fn reg(&mut self, reg: Register) -> &mut u32 {
        match reg {
            Register::A => &mut self.a,
            Register::B => &mut self.b,
        }
    }
    fn exeucte_instr(&mut self, instr: Instr) {
        match instr {
            Instr::Hlf(r) => *self.reg(r) /= 2,
            Instr::Tpl(r) => *self.reg(r) *= 3,
            Instr::Inc(r) => *self.reg(r) += 1,
            Instr::Jmp(offset) => self.ip += offset - 1,
            Instr::Jie(r, offset) => {
                if *self.reg(r) % 2 == 0 {
                    self.ip += offset - 1;
                }
            }
            Instr::Jio(r, offset) => {
                if *self.reg(r) == 1 {
                    self.ip += offset - 1;
                }
            }
        }
        self.ip += 1;
    }
    fn run(&mut self, instrs: &[Instr]) {
        while let Some(instr) = self.ip.try_into().ok().and_then(|ip: usize| instrs.get(ip)) {
            self.exeucte_instr(*instr);
        }
    }
}

#[derive(Clone, Copy)]
enum Instr {
    Hlf(Register),
    Tpl(Register),
    Inc(Register),
    Jmp(i32),
    Jie(Register, i32),
    Jio(Register, i32),
}

#[derive(Clone, Copy)]
enum Register {
    A,
    B,
}

fn parse_reg(r: &str) -> Register {
    match r {
        "a" => Register::A,
        "b" => Register::B,
        _ => panic!(),
    }
}

fn parse_instr(input: &str) -> Instr {
    let (name, args_str) = input.split_once(' ').unwrap();
    let args = args_str.split(", ").collect_vec();
    match (name, &args[..]) {
        ("hlf", [r]) => Instr::Hlf(parse_reg(r)),
        ("tpl", [r]) => Instr::Tpl(parse_reg(r)),
        ("inc", [r]) => Instr::Inc(parse_reg(r)),
        ("jmp", [offset]) => Instr::Jmp(offset.parse().unwrap()),
        ("jie", [r, offset]) => Instr::Jie(parse_reg(r), offset.parse().unwrap()),
        ("jio", [r, offset]) => Instr::Jio(parse_reg(r), offset.parse().unwrap()),
        _ => panic!(),
    }
}

fn part1(input: &str) -> u32 {
    let instrs = input.lines().map(parse_instr).collect_vec();
    let mut computer = Computer::new();
    computer.run(&instrs);
    computer.b
}

fn part2(input: &str) -> u32 {
    let instrs = input.lines().map(parse_instr).collect_vec();
    let mut computer = Computer::new();
    computer.a = 1;
    computer.run(&instrs);
    computer.b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day() {
        let input = include_str!("./input.txt").trim();
        assert_eq!(part1(input), 170);
        assert_eq!(part2(input), 247);
    }
}
