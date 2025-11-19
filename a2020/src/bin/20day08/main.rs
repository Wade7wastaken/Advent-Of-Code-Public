use lib::itertools::Itertools;

fn main() {
    let input = include_str!("./input.txt").trim();
    println!("{}", part1(input));
    // println!("{}", part2(input));
}


enum Operation {
    Acc(i32),
    Jmp(i32),
    Nop,
}

struct Instruction {
    op: Operation,
    seen_before: bool,
}

fn parse_instr(l: &str) -> Instruction {
    let (name, value) = l.split_once(' ').unwrap();

    let op = match name {
        "acc" => Operation::Acc(value.parse().unwrap()),
        "jmp" => Operation::Jmp(value.parse().unwrap()),
        "nop" => Operation::Nop,
        _ => panic!(),
    };

    Instruction {
        op,
        seen_before: false,
    }
}

fn run(instrs: &mut [Instruction]) -> Result<i32, i32> {
    let mut pc = 0;
    let mut acc = 0;

    loop {
        if let Some(instr) = instrs.get_mut(pc) {
            pc += 1;
            if instr.seen_before {
                return Err(acc);
            }
            instr.seen_before = true;
            match instr.op {
                Operation::Acc(arg) => acc += arg,
                Operation::Jmp(arg) => pc = (pc as i32 + arg - 1) as usize,
                Operation::Nop => {}
            }
        } else {
            return Ok(acc);
        }
    }
}

fn part1(input: &str) -> i32 {
    let mut instrs = input.lines().map(parse_instr).collect_vec();
    let mut pc = 0;
    let mut acc = 0;

    loop {
        let instr = instrs.get_mut(pc).unwrap();
        pc += 1;
        if instr.seen_before {
            return acc;
        }
        instr.seen_before = true;
        match instr.op {
            Operation::Acc(arg) => acc += arg,
            Operation::Jmp(arg) => pc = (pc as i32 + arg - 1) as usize,
            Operation::Nop => {}
        }
    }
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
