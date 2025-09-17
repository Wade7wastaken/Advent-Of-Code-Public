use lib::{StringTools, itertools::Itertools};

fn main() {
    let input = include_str!("./input.txt").trim();
    println!("{}", part1(input));
    println!("{}", part2(input));
}

struct Instr {
    opcode: u64,
    operand: u64,
}

struct Computer {
    a: u64,
    b: u64,
    c: u64,
    ip: usize,
    program: Vec<Instr>,
}

fn parse_registers(input: &str) -> (u64, u64, u64) {
    input.lines().map(parse_register).collect_tuple().unwrap()
}

fn parse_register(input: &str) -> u64 {
    input.split_once(": ").unwrap().1.parse().unwrap()
}

fn parse_source(input: &str) -> Vec<u64> {
    input
        .split_once(": ")
        .unwrap()
        .1
        .split(',')
        .map(|n| n.parse().unwrap())
        .collect_vec()
}

impl Computer {
    fn new((a, b, c): (u64, u64, u64), source: Vec<u64>) -> Self {
        let program = source
            .into_iter()
            .tuples()
            .map(|(opcode, operand)| Instr { opcode, operand })
            .collect();

        Self {
            a,
            b,
            c,
            ip: 0,
            program,
        }
    }

    fn combo(&self, operand: u64) -> u64 {
        match operand {
            0..=3 => operand,
            4 => self.a,
            5 => self.b,
            6 => self.c,
            _ => 0,
        }
    }

    fn run(&mut self) -> Vec<u64> {
        let mut output = vec![];

        while let Some(instr) = self.program.get(self.ip) {
            let mut jump = true;
            let combo = self.combo(instr.operand);
            match instr.opcode {
                // adv
                0 => self.a /= 2u64.pow(self.combo(instr.operand) as u32),
                // bxl
                1 => self.b ^= instr.operand,
                // bst
                2 => self.b = self.combo(instr.operand) % 8,
                // jnz
                3 => {
                    if self.a != 0 {
                        self.ip = instr.operand as usize;
                        jump = false;
                    }
                }
                // bxc
                4 => self.b ^= self.c,
                // out
                5 => output.push(combo % 8),
                // bdv
                6 => self.b = self.a / 2u64.pow(self.combo(instr.operand) as u32),
                // cdv
                7 => self.c = self.a / 2u64.pow(self.combo(instr.operand) as u32),
                _ => unreachable!(),
            }
            if jump {
                self.ip += 1;
            }
        }

        output
    }
}

fn part1(input: &str) -> String {
    let (registers_str, program_str) = input.split_paragraphs_once().unwrap();

    let registers = parse_registers(registers_str);
    let source = parse_source(program_str);

    Computer::new(registers, source)
        .run()
        .into_iter()
        .map(|n| n.to_string())
        .join(",")
}

fn find_digits(source: &Vec<u64>, i: usize, prev: u64) -> Option<u64> {
    for a in 0..8 {
        let starting_a = prev + (a * 8u64.pow(i as u32));
        let output = Computer::new((starting_a, 0, 0), source.clone()).run();

        if output.get(i) != source.get(i) {
            continue;
        }
        if i == 0 {
            return Some(starting_a);
        }
        if let Some(next) = find_digits(source, i - 1, starting_a) {
            return Some(next);
        }
    }
    None
}

fn part2(input: &str) -> u64 {
    let source = parse_source(input.split_paragraphs_once().unwrap().1);

    let len = source.len() - 1;

    find_digits(&source, len, 0).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day() {
        let input = include_str!("./input.txt").trim();
        assert_eq!(part1(input), "5,0,3,5,7,6,1,5,4");
        assert_eq!(part2(input), 164516454365621);
    }
}
