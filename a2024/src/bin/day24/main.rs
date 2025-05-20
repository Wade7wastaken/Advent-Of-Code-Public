use std::{
    collections::{HashMap, HashSet},
    mem::swap,
};

use lib::{itertools::Itertools, StringTools};

fn main() {
    let input = include_str!("./input.txt").trim();
    // println!("{}", part1(input));
    println!("{}", part2(input));
}

enum Wire {
    Active(bool),
    Inactive {
        gate: Gate,
        input_1: String,
        input_2: String,
    },
}

// struct Wire {
//     value: Option<bool>,
//     gate: Gate,
//     input1: String,
//     input2: String,
// }

enum Gate {
    And,
    Or,
    Xor,
}

fn parse_gate(s: &str) -> Gate {
    match s {
        "AND" => Gate::And,
        "OR" => Gate::Or,
        "XOR" => Gate::Xor,
        _ => panic!("unknown gate {s}"),
    }
}

fn wire_value(wires: &mut HashMap<&str, Wire>, w: &str) -> bool {
    let a = wires.get_mut(w).unwrap();

    match a {
        Wire::Active(v) => *v,
        Wire::Inactive { gate, input_1, input_2 } => {
            // let v1 = wire_value(wires, &input_1);
            // let v2 = wire_value(wires, &input_2);


            true
        }
    }
}

fn part1(input: &str) {
    let (initial_str, gates_str) = input.split_paragraphs_once().unwrap();

    let mut wires = HashMap::new();

    for initial in initial_str.lines() {
        let (name, value_str) = initial.split_once(": ").unwrap();
        wires.insert(name, Wire::Active(value_str == "1"));
    }

    for gate in gates_str.lines() {
        let (input_1, gate_str, input_2, _, output) =
            gate.split_ascii_whitespace().collect_tuple().unwrap();
        wires.insert(
            output,
            Wire::Inactive {
                gate: parse_gate(gate_str),
                input_1: input_1.to_string(),
                input_2: input_2.to_string(),
            },
        );
    }
}

fn format_xyz(bit: u32, prefix: &str) -> String {
    format!("{prefix}{bit:02}")
}

fn part1_old(input: &str) -> u32 {
    let (initial_str, gates_str) = input.split_paragraphs_once().unwrap();

    let mut src_to_dest = HashMap::new();
    let mut dest_to_src = HashMap::new();

    for (input_1, gate, input_2, _, output) in gates_str
        .lines()
        .map(|l| l.split_whitespace().collect_tuple().unwrap())
    {
        let set = [input_1, input_2].into_iter().collect::<HashSet<_>>();
        src_to_dest.insert((input_1, gate, input_2), output);
        src_to_dest.insert((input_2, gate, input_1), output);
        dest_to_src.insert(output, (set.clone(), gate));
        dest_to_src.insert(output, (set, gate));
    }

    let mut carry_in = src_to_dest.get(&("x00", "AND", "y00")).unwrap();

    for bit in 1..45 {
        println!("bit {bit} carry in {carry_in}");
        let x = format_xyz(bit, "x");
        let y = format_xyz(bit, "y");
        let z = format_xyz(bit, "z");
        let t1 = src_to_dest.get(&(&x, "XOR", &y)).unwrap();
        let t2 = src_to_dest.get(&(&x, "AND", &y)).unwrap();

        let (also_t1_carry, should_be_xor) = dest_to_src.get(&z.as_str()).unwrap();
        assert_eq!(
            also_t1_carry,
            &[*t1, *carry_in].into_iter().collect::<HashSet<_>>()
        );
        assert_eq!(should_be_xor, &"XOR");

        let t3 = src_to_dest.get(&(t1, "AND", carry_in)).unwrap();
        carry_in = src_to_dest.get(&(t3, "OR", t2)).unwrap();
    }

    println!("{}", carry_in);

    4
}

fn part2(input: &str) -> String {
    ["swt", "z07", "pqc", "z13", "wsv", "rjm", "bgs", "z31"]
        .into_iter()
        .sorted()
        .join(",")
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
